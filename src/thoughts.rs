use crate::{
    cards::{ActivityCard, Mood, ResourceType, TimeOfDay},
    logic::{
        CardSelectedEvent, DayChangedEvent, GamePhase, GamePhaseState, GameState, MoodChangedEvent,
        PhaseChangedEvent, ResourceChangedEvent, TimeChangedEvent,
    },
};
use bevy::prelude::*;
use bevy_llm::*;
use std::collections::VecDeque;

pub struct CharacterThoughtsPlugin;

impl Plugin for CharacterThoughtsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ThoughtGenerationSystem>()
            .init_resource::<ActionLog>()
            .add_event::<GenerateThoughtEvent>()
            .add_event::<ThoughtGeneratedEvent>()
            .add_systems(
                Update,
                (
                    process_thought_generation,
                    handle_llm_responses,
                    handle_async_llm_responses,
                    // Event listeners that trigger thoughts automatically
                    listen_for_mood_changes,
                    listen_for_resource_crises,
                    listen_for_time_changes,
                    listen_for_phase_changes,
                    listen_for_card_selections,
                ),
            );
    }
}

// Simplified thought generation system
#[derive(Resource)]
pub struct ThoughtGenerationSystem {
    pub pending_requests: std::collections::HashMap<u32, ThoughtContext>,
    pub streaming_thoughts: std::collections::HashMap<u32, String>, // Accumulate tokens
    pub next_request_id: u32,
}

impl Default for ThoughtGenerationSystem {
    fn default() -> Self {
        Self {
            pending_requests: std::collections::HashMap::new(),
            streaming_thoughts: std::collections::HashMap::new(),
            next_request_id: 1,
        }
    }
}

// Action logging system
#[derive(Resource)]
pub struct ActionLog {
    pub recent_actions: VecDeque<ActionEntry>,
    pub max_history: usize,
}

impl Default for ActionLog {
    fn default() -> Self {
        Self {
            recent_actions: VecDeque::new(),
            max_history: 10, // Keep last 10 actions for context
        }
    }
}

#[derive(Clone, Debug)]
pub struct ActionEntry {
    pub action_type: ActionType,
    pub description: String,
    pub timestamp: f32, // Game hour
    pub day: u32,
    pub mood_at_time: Mood,
}

#[derive(Clone, Debug)]
pub enum ActionType {
    CardPlayed(ActivityCard),  // Card name
    ObjectInteraction(String), // Object name
    ResourceCrisis(ResourceType),
    MoodChange(Mood, Mood), // From, To
}

// Main event to trigger thought generation
#[derive(Event)]
pub struct GenerateThoughtEvent {
    pub thought_type: ThoughtType,
    pub context: Option<String>, // Optional additional context
}

#[derive(Event)]
pub struct ThoughtGeneratedEvent {
    pub text: String,
}

#[derive(Clone, Debug)]
pub struct ThoughtContext {
    pub thought_type: ThoughtType,
    pub current_state: GameStateSnapshot,
    pub recent_actions: Vec<ActionEntry>,
    pub additional_context: Option<String>,
}

#[derive(Clone, Debug)]
pub enum ThoughtType {
    CardPlayed(ActivityCard),     // Card name
    ObjectInteraction(String),    // Object name
    ResourceCrisis(ResourceType), // Which resource is low
    MoodChange(Mood, Mood),       // From mood, to mood
    TimeChange(TimeOfDay),
    // New time of day
    PhaseChange(GamePhase), // New game phase
    General,                // General ambient thought
}

#[derive(Clone, Debug)]
pub struct GameStateSnapshot {
    pub sleep: f32,
    pub health: f32,
    pub mental_health: f32,
    pub food: f32,
    pub current_hour: f32,
    pub current_day: u32,
    pub current_mood: Mood,
    pub time_of_day: TimeOfDay,
    pub current_phase: GamePhase,
    pub turn_number: u32,
}

impl GameStateSnapshot {
    pub fn from_game_state(game_state: &GameState, phase_state: &GamePhaseState) -> Self {
        Self {
            sleep: game_state.sleep,
            health: game_state.health,
            mental_health: game_state.mental_health,
            food: game_state.food,
            current_hour: game_state.current_hour,
            current_day: game_state.current_day,
            current_mood: game_state.current_mood,
            time_of_day: game_state.time_of_day,
            current_phase: phase_state.current_phase,
            turn_number: phase_state.turn_number,
        }
    }
}

// Process thought generation events
fn process_thought_generation(
    mut thought_events: EventReader<GenerateThoughtEvent>,
    mut thought_system: ResMut<ThoughtGenerationSystem>,
    mut llm_requests: EventWriter<AiGenerationRequest>,
    game_state: Res<GameState>,
    phase_state: Res<GamePhaseState>,
    action_log: Res<ActionLog>,
) {
    for event in thought_events.read() {
        let request_id = thought_system.next_request_id;
        thought_system.next_request_id += 1;

        let context = ThoughtContext {
            thought_type: event.thought_type.clone(),
            current_state: GameStateSnapshot::from_game_state(&game_state, &phase_state),
            recent_actions: action_log.get_recent_actions_summary(5),
            additional_context: event.context.clone(),
        };

        let prompt = generate_thought_prompt(&context);

        let messages = vec![
            ChatMessage::system(&get_character_system_prompt()),
            ChatMessage::user(&prompt),
        ];

        let request = AiGenerationRequest::with_config(
            request_id,
            messages,
            Some(80),  // Max tokens for thoughts
            Some(0.8), // Temperature for varied thoughts
        );

        thought_system.pending_requests.insert(request_id, context);
        llm_requests.write(request);
    }
}

fn handle_llm_responses(
    mut llm_responses: EventReader<AiGenerationResponse>,
    mut thought_system: ResMut<ThoughtGenerationSystem>,
    mut update_thoughts: EventWriter<ThoughtGeneratedEvent>,
) {
    for response in llm_responses.read() {
        if let Some(_context) = thought_system.pending_requests.remove(&response.id) {
            // Clean up the response text
            let thought = response.result.trim().to_string();

            update_thoughts.write(ThoughtGeneratedEvent { text: thought });
        }
    }
}

fn handle_async_llm_responses(
    mut async_responses: EventReader<AsyncAiGenerationResponse>,
    mut thought_system: ResMut<ThoughtGenerationSystem>,
    mut update_thoughts: EventWriter<ThoughtGeneratedEvent>,
) {
    for response in async_responses.read() {
        // Check if this is a request we're tracking
        if thought_system.pending_requests.contains_key(&response.id) {
            // Accumulate the new token
            let accumulated = thought_system
                .streaming_thoughts
                .entry(response.id)
                .or_insert_with(String::new);

            accumulated.push_str(&response.result);

            // Send the current accumulated thought to UI
            // This will update the display with each new token
            update_thoughts.write(ThoughtGeneratedEvent {
                text: accumulated.clone(),
            });

            // Note: We don't remove from pending_requests here because
            // async responses are incremental. The final response will
            // be handled by handle_llm_responses or we need to detect
            // when streaming is complete based on your LLM library's behavior
        }
    }
}

// Action log helper methods
impl ActionLog {
    pub fn log_action(
        &mut self,
        action_type: ActionType,
        description: String,
        game_state: &GameState,
    ) {
        let entry = ActionEntry {
            action_type,
            description,
            timestamp: game_state.current_hour,
            day: game_state.current_day,
            mood_at_time: game_state.current_mood,
        };

        self.recent_actions.push_back(entry);

        // Keep only recent actions
        while self.recent_actions.len() > self.max_history {
            self.recent_actions.pop_front();
        }
    }

    pub fn get_recent_actions_summary(&self, max_count: usize) -> Vec<ActionEntry> {
        self.recent_actions
            .iter()
            .rev()
            .take(max_count)
            .cloned()
            .collect()
    }
}

// Generate the system prompt for the hikikomori character
fn get_character_system_prompt() -> String {
    r#"You are a hikikomori character - someone who has withdrawn from society and rarely leaves their apartment. Your thoughts should reflect:

PERSONALITY:
- Social anxiety and fear of judgment
- Depression mixed with moments of hope
- Overthinking and catastrophizing
- Self-awareness about your situation
- Comfort in routines, anxiety about change

THOUGHT STYLE:
- Write in first person ("I think...", "Maybe I should...")
- Keep to 1-2 sentences maximum
- Use natural, internal monologue
- Show vulnerability and self-doubt
- Occasionally have clarity or determination

Respond with a realistic internal thought for the given situation."#.to_string()
}

// Generate context-specific prompts
fn generate_thought_prompt(context: &ThoughtContext) -> String {
    let state = &context.current_state;
    let base_context = format!(
        "Day {}, {:02}:{:02}, Mood: {:?}. Sleep {:.0}%, Health {:.0}%, Mental {:.0}%, Food {:.0}%.",
        state.current_day,
        state.current_hour as u32,
        ((state.current_hour % 1.0) * 60.0) as u32,
        state.current_mood,
        state.sleep,
        state.health,
        state.mental_health,
        state.food
    );

    let specific_prompt = match &context.thought_type {
        ThoughtType::CardPlayed(card) => {
            format!(
                "I just chose '{} : {}'. What am I thinking?",
                card.name, card.description
            )
        }
        ThoughtType::ObjectInteraction(object) => {
            format!("I'm looking at the {}. What crosses my mind?", object)
        }
        ThoughtType::ResourceCrisis(resource) => {
            format!(
                "My {:?} is critically low. What desperate thoughts am I having?",
                resource
            )
        }
        ThoughtType::MoodChange(from, to) => {
            format!(
                "My mood shifted from {:?} to {:?}. How do I feel?",
                from, to
            )
        }
        ThoughtType::TimeChange(time_of_day) => {
            format!(
                "It's {:?} now. What thoughts does this time bring?",
                time_of_day
            )
        }
        ThoughtType::PhaseChange(phase) => {
            format!("Game phase is {:?}. What am I thinking?", phase)
        }
        ThoughtType::General => {
            "I'm sitting here in my apartment. What random thought drifts through my mind?"
                .to_string()
        }
    };

    let additional = if let Some(extra) = &context.additional_context {
        format!(" Additional context: {}", extra)
    } else {
        String::new()
    };

    format!("{} {}{}", base_context, specific_prompt, additional)
}

// Automatic event listeners that integrate with GameLogicPlugin events
fn listen_for_mood_changes(
    mut mood_events: EventReader<MoodChangedEvent>,
    mut thought_events: EventWriter<GenerateThoughtEvent>,
    mut action_log: ResMut<ActionLog>,
    game_state: Res<GameState>,
) {
    for event in mood_events.read() {
        // Log the mood change
        action_log.log_action(
            ActionType::MoodChange(event.old_mood, event.new_mood),
            format!(
                "Mood changed from {:?} to {:?}",
                event.old_mood, event.new_mood
            ),
            &game_state,
        );

        // Generate thought about mood change
        // thought_events.write(GenerateThoughtEvent {
        //     thought_type: ThoughtType::MoodChange(event.old_mood, event.new_mood),
        //     context: None,
        // });
    }
}

fn listen_for_resource_crises(
    mut resource_events: EventReader<ResourceChangedEvent>,
    mut thought_events: EventWriter<GenerateThoughtEvent>,
    mut action_log: ResMut<ActionLog>,
    game_state: Res<GameState>,
    mut last_crisis_states: Local<std::collections::HashMap<ResourceType, bool>>,
) {
    for event in resource_events.read() {
        let is_crisis = event.new_value < 20.0;
        let was_crisis = last_crisis_states
            .get(&event.resource_type)
            .copied()
            .unwrap_or(false);

        // Only trigger on new crises (not ongoing ones)
        if is_crisis && !was_crisis {
            // Log the crisis
            action_log.log_action(
                ActionType::ResourceCrisis(event.resource_type),
                format!(
                    "{:?} reached crisis level: {:.1}",
                    event.resource_type, event.new_value
                ),
                &game_state,
            );

            // Generate desperate thought
            // thought_events.write(GenerateThoughtEvent {
            //     thought_type: ThoughtType::ResourceCrisis(event.resource_type),
            //     context: Some(format!("Only {:.0}% left", event.new_value)),
            // });
        }

        last_crisis_states.insert(event.resource_type, is_crisis);
    }
}

fn listen_for_time_changes(
    mut time_events: EventReader<TimeChangedEvent>,
    mut thought_events: EventWriter<GenerateThoughtEvent>,
) {
    for event in time_events.read() {
        // Only generate thoughts when time of day changes (not every hour)
        if event.old_time_of_day != event.new_time_of_day {
            // thought_events.write(GenerateThoughtEvent {
            //     thought_type: ThoughtType::TimeChange(event.new_time_of_day),
            //     context: None,
            // });
        }
    }
}

fn listen_for_phase_changes(
    mut phase_events: EventReader<PhaseChangedEvent>,
    mut thought_events: EventWriter<GenerateThoughtEvent>,
) {
    for event in phase_events.read() {
        // Generate thoughts for all phase changes
        match event.new_phase {
            GamePhase::CardDraw => {
                // New turn starting
                thought_events.write(GenerateThoughtEvent {
                    thought_type: ThoughtType::PhaseChange(event.new_phase),
                    context: Some("Another turn, another choice to make".to_string()),
                });
            }
            GamePhase::CardSelection => {
                // Looking at available options
                thought_events.write(GenerateThoughtEvent {
                    thought_type: ThoughtType::PhaseChange(event.new_phase),
                    context: Some("What should I do now? These choices all seem...".to_string()),
                });
            }
            GamePhase::CharacterAction => {
                // About to act on the choice
                thought_events.write(GenerateThoughtEvent {
                    thought_type: ThoughtType::PhaseChange(event.new_phase),
                    context: Some("Here I go again, doing the same things...".to_string()),
                });
            }
            GamePhase::TurnOver => {}
        }
    }
}

fn listen_for_card_selections(
    mut card_events: EventReader<CardSelectedEvent>,
    mut thought_events: EventWriter<GenerateThoughtEvent>,
    mut action_log: ResMut<ActionLog>,
    game_state: Res<GameState>,
) {
    for event in card_events.read() {
        let card = event.0.clone();

        action_log.log_action(
            ActionType::CardPlayed(card.clone()),
            format!("Selected card: {}", card.name),
            &game_state,
        );

        // Generate thought about the card choice
        thought_events.write(GenerateThoughtEvent {
            thought_type: ThoughtType::CardPlayed(card.clone()),
            context: None,
        });
    }
}
