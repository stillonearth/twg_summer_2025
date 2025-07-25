use bevy::prelude::*;
use bevy_novel::{events::EventStartScenario, rpy_asset_loader::Rpy};

use crate::{cards::ActivityCard, cutscene::ScenarioHandle};

pub struct GameLogicPlugin;

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameState>()
            .init_resource::<GamePhaseState>()
            .add_event::<GameStepEvent>()
            .add_event::<ResourceChangedEvent>()
            .add_event::<MoodChangedEvent>()
            .add_event::<TimeChangedEvent>()
            .add_event::<DayChangedEvent>()
            .add_event::<PhaseChangedEvent>()
            .add_event::<CardDrawnEvent>()
            .add_event::<CardSelectedEvent>()
            .add_event::<ActionCompletedEvent>()
            .add_event::<CutsceneStartEvent>()
            .add_event::<CutsceneEndEvent>()
            .add_systems(
                Update,
                (
                    handle_game_step_events,
                    handle_phase_transitions,
                    handle_card_draw,
                    handle_card_selection,
                    handle_action_completion,
                    handle_character_action_phase,
                    handle_cutscene_trigger,
                    handle_cutscene_end,
                ),
            );
    }
}

#[derive(Event, Deref)]
pub struct CardSelectedEvent(pub ActivityCard);

#[derive(Event)]
pub struct CutsceneEndEvent;

#[derive(Event)]
pub struct PhaseChangedEvent {
    pub old_phase: GamePhase,
    pub new_phase: GamePhase,
}

#[derive(Event)]
pub struct CardDrawnEvent {
    pub card_count: usize,
}

#[derive(Event)]
pub struct ActionCompletedEvent {}

#[derive(Event)]
pub struct CutsceneStartEvent {
    pub cutscene_id: String,
    pub trigger_reason: CutsceneTrigger,
}

// Existing Events
#[derive(Event)]
pub struct GameStepEvent {
    pub time_delta: f32,
    pub sleep_change: f32,
    pub health_change: f32,
    pub mental_health_change: f32,
    pub food_change: f32,
}

#[derive(Event)]
pub struct ResourceChangedEvent {
    pub resource_type: ResourceType,
    pub old_value: f32,
    pub new_value: f32,
}

#[derive(Event)]
pub struct MoodChangedEvent {
    pub old_mood: Mood,
    pub new_mood: Mood,
}

#[derive(Event)]
pub struct TimeChangedEvent {
    pub old_hour: f32,
    pub new_hour: f32,
    pub old_time_of_day: TimeOfDay,
    pub new_time_of_day: TimeOfDay,
}

#[derive(Event)]
pub struct DayChangedEvent {
    pub old_day: u32,
    pub new_day: u32,
}

// Game Phase System - Removed VisualNovelCutscene
#[derive(Resource)]
pub struct GamePhaseState {
    pub current_phase: GamePhase,
    pub previous_phase: Option<GamePhase>, // Track phase before cutscene
    pub turn_number: u32,
    pub cards_drawn_count: usize,
    pub selected_card_number: Option<usize>,
    pub pending_cutscene: Option<String>,
    pub cutscene_active: bool, // Track if cutscene is running
}

impl Default for GamePhaseState {
    fn default() -> Self {
        Self {
            current_phase: GamePhase::CardDraw,
            previous_phase: None,
            turn_number: 1,
            cards_drawn_count: 0,
            selected_card_number: None,
            pending_cutscene: None,
            cutscene_active: false,
        }
    }
}

// Removed VisualNovelCutscene from enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GamePhase {
    CardDraw,
    CardSelection,
    CharacterAction,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CutsceneTrigger {
    CardEffect,
    CharacterAction,
    MoodChange,
    TimeOfDay,
    ResourceThreshold,
}

// Core game state resource (existing, unchanged)
#[derive(Resource)]
pub struct GameState {
    // Time
    pub current_hour: f32,
    pub current_day: u32,
    pub time_speed: f32,

    // Resources
    pub sleep: f32,
    pub health: f32,
    pub mental_health: f32,
    pub food: f32,

    // Derived states
    pub current_mood: Mood,
    pub time_of_day: TimeOfDay,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            current_hour: 10.0,
            current_day: 1,
            time_speed: 1.0,
            sleep: 70.0,
            health: 80.0,
            mental_health: 60.0,
            food: 50.0,
            current_mood: Mood::Neutral,
            time_of_day: TimeOfDay::Morning,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mood {
    Depressed,
    Anxious,
    Tired,
    Neutral,
    Content,
    Manic,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimeOfDay {
    EarlyMorning, // 5-9
    Morning,      // 9-12
    Afternoon,    // 12-17
    Evening,      // 17-20
    Night,        // 20-24
    LateNight,    // 0-5
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ResourceType {
    Sleep,
    Health,
    Mental,
    Food,
}

impl GameState {
    fn calculate_time_of_day(hour: f32) -> TimeOfDay {
        match hour {
            h if (5.0..9.0).contains(&h) => TimeOfDay::EarlyMorning,
            h if (9.0..12.0).contains(&h) => TimeOfDay::Morning,
            h if (12.0..17.0).contains(&h) => TimeOfDay::Afternoon,
            h if (17.0..20.0).contains(&h) => TimeOfDay::Evening,
            h if !(5.0..20.0).contains(&h) => {
                if h >= 20.0 {
                    TimeOfDay::Night
                } else {
                    TimeOfDay::LateNight
                }
            }
            _ => TimeOfDay::Morning,
        }
    }

    fn calculate_mood(&self) -> Mood {
        let avg_resources = (self.sleep + self.health + self.mental_health + self.food) / 4.0;

        // Factor in specific resource states
        match () {
            _ if self.mental_health < 20.0 => Mood::Depressed,
            _ if self.sleep < 20.0 => Mood::Tired,
            _ if self.mental_health < 40.0 && self.sleep < 40.0 => Mood::Anxious,
            _ if avg_resources > 80.0 => Mood::Content,
            _ if self.mental_health > 90.0 && self.sleep < 30.0 => Mood::Manic,
            _ => Mood::Neutral,
        }
    }

    pub fn get_time_string(&self) -> String {
        let hour = self.current_hour as u32;
        let minute = ((self.current_hour % 1.0) * 60.0) as u32;
        format!("{hour:02}:{minute:02}")
    }

    pub fn get_day_string(&self) -> String {
        format!("Day {}", self.current_day)
    }

    pub fn get_resource_value(&self, resource_type: ResourceType) -> f32 {
        match resource_type {
            ResourceType::Sleep => self.sleep,
            ResourceType::Health => self.health,
            ResourceType::Mental => self.mental_health,
            ResourceType::Food => self.food,
        }
    }
}

// Phase transition system
fn handle_phase_transitions(
    mut phase_state: ResMut<GamePhaseState>,
    mut phase_changed_events: EventWriter<PhaseChangedEvent>,
) {
    // This system would be triggered by UI interactions or automatic progression
    // For now, it's a placeholder for the phase transition logic
}

// Card draw phase system
fn handle_card_draw(
    mut card_drawn_events: EventReader<CardDrawnEvent>,
    mut phase_state: ResMut<GamePhaseState>,
    mut phase_changed_events: EventWriter<PhaseChangedEvent>,
) {
    for event in card_drawn_events.read() {
        // Don't transition if cutscene is active
        if phase_state.cutscene_active {
            continue;
        }

        phase_state.cards_drawn_count = event.card_count;

        let old_phase = phase_state.current_phase;
        phase_state.current_phase = GamePhase::CardSelection;

        phase_changed_events.write(PhaseChangedEvent {
            old_phase,
            new_phase: phase_state.current_phase,
        });
    }
}

// Card selection phase system
fn handle_card_selection(
    mut card_selected_events: EventReader<CardSelectedEvent>,
    mut phase_state: ResMut<GamePhaseState>,
    mut phase_changed_events: EventWriter<PhaseChangedEvent>,
    mut cutscene_events: EventWriter<CutsceneStartEvent>,
) {
    for event in card_selected_events.read() {
        // Don't transition if cutscene is active
        if phase_state.cutscene_active {
            continue;
        }

        phase_state.selected_card_number = Some(event.card_number);

        // Check if cutscene should be triggered
        let has_cutscene = false; // Your cards plugin would determine this

        if has_cutscene {
            // Trigger cutscene instead of transitioning
            cutscene_events.write(CutsceneStartEvent {
                cutscene_id: "card_action".to_string(),
                trigger_reason: CutsceneTrigger::CardEffect,
            });
        } else {
            // Normal transition to character action
            let old_phase = phase_state.current_phase;
            phase_state.current_phase = GamePhase::CharacterAction;

            phase_changed_events.write(PhaseChangedEvent {
                old_phase,
                new_phase: phase_state.current_phase,
            });
        }
    }
}

// Updated cutscene trigger - doesn't change phase, just sets cutscene state
fn handle_cutscene_trigger(
    mut cutscene_events: EventReader<CutsceneStartEvent>,
    mut phase_state: ResMut<GamePhaseState>,
    mut novel_events: EventWriter<EventStartScenario>,
    scenario: Res<ScenarioHandle>,
    rpy_assets: Res<Assets<Rpy>>,
) {
    for event in cutscene_events.read() {
        // Store the cutscene ID and mark cutscene as active
        phase_state.pending_cutscene = Some(event.cutscene_id.clone());
        phase_state.previous_phase = Some(phase_state.current_phase);
        phase_state.cutscene_active = true;

        info!(
            "Starting cutscene: {} during {:?} phase",
            event.cutscene_id, phase_state.current_phase
        );

        // Start the visual novel cutscene
        if let Some(rpy) = rpy_assets.get(scenario.id()) {
            novel_events.write(EventStartScenario { ast: rpy.0.clone() });
        }
    }
}

// Updated cutscene end - resumes previous phase
fn handle_cutscene_end(
    mut cutscene_end_events: EventReader<CutsceneEndEvent>,
    mut phase_state: ResMut<GamePhaseState>,
    mut phase_changed_events: EventWriter<PhaseChangedEvent>,
    mut action_completed_events: EventWriter<ActionCompletedEvent>,
) {
    for _event in cutscene_end_events.read() {
        if !phase_state.cutscene_active {
            continue;
        }

        info!("Cutscene ended, resuming game phase");

        // Clear cutscene state
        phase_state.cutscene_active = false;
        phase_state.pending_cutscene = None;

        // Determine what to do based on the previous phase and current state
        match phase_state.current_phase {
            GamePhase::CardSelection => {
                // If we were in card selection and had a cutscene,
                // now proceed to character action
                if phase_state.selected_card_number.is_some() {
                    let old_phase = phase_state.current_phase;
                    phase_state.current_phase = GamePhase::CharacterAction;

                    phase_changed_events.write(PhaseChangedEvent {
                        old_phase,
                        new_phase: phase_state.current_phase,
                    });
                }
            }
            GamePhase::CharacterAction => {
                // If we were in character action, complete the action
                action_completed_events.write(ActionCompletedEvent {});
            }
            GamePhase::CardDraw => {
                // Nothing special needed for card draw phase
            }
        }

        phase_state.previous_phase = None;
    }
}

// Action completion system
fn handle_action_completion(
    mut action_completed_events: EventReader<ActionCompletedEvent>,
    mut phase_state: ResMut<GamePhaseState>,
    mut phase_changed_events: EventWriter<PhaseChangedEvent>,
) {
    for _event in action_completed_events.read() {
        // Don't transition if cutscene is active
        if phase_state.cutscene_active {
            continue;
        }

        // Clear phase state
        phase_state.selected_card_number = None;
        phase_state.cards_drawn_count = 0;

        // Increment turn and go back to card draw
        phase_state.turn_number += 1;

        let old_phase = phase_state.current_phase;
        phase_state.current_phase = GamePhase::CardDraw;

        phase_changed_events.write(PhaseChangedEvent {
            old_phase,
            new_phase: phase_state.current_phase,
        });
    }
}

fn handle_character_action_phase(
    mut phase_changed_events: EventReader<PhaseChangedEvent>,
    mut action_completed_events: EventWriter<ActionCompletedEvent>,
    mut ew_start_start: EventWriter<CutsceneStartEvent>,
) {
    for event in phase_changed_events.read() {
        if event.new_phase == GamePhase::CharacterAction {
            info!("Entered Character Action phase");

            // For now, immediately complete the action
            // action_completed_events.write(ActionCompletedEvent {});
            //
            ew_start_start.write(CutsceneStartEvent {
                cutscene_id: "test".to_string(),
                trigger_reason: CutsceneTrigger::CharacterAction,
            });
        }
    }
}

// Existing event handler system (unchanged)
fn handle_game_step_events(
    mut game_step_events: EventReader<GameStepEvent>,
    mut resource_changed_events: EventWriter<ResourceChangedEvent>,
    mut mood_changed_events: EventWriter<MoodChangedEvent>,
    mut time_changed_events: EventWriter<TimeChangedEvent>,
    mut day_changed_events: EventWriter<DayChangedEvent>,
    mut game_state: ResMut<GameState>,
) {
    for event in game_step_events.read() {
        // Update time
        let old_hour = game_state.current_hour;
        let old_day = game_state.current_day;
        let old_time_of_day = game_state.time_of_day;

        game_state.current_hour += event.time_delta * game_state.time_speed * (1.0 / 3600.0);

        // Handle day rollover
        if game_state.current_hour >= 24.0 {
            game_state.current_hour -= 24.0;
            game_state.current_day += 1;

            day_changed_events.write(DayChangedEvent {
                old_day,
                new_day: game_state.current_day,
            });
        }

        // Update time of day
        game_state.time_of_day = GameState::calculate_time_of_day(game_state.current_hour);

        // Send time changed event if anything changed
        if old_hour != game_state.current_hour || old_time_of_day != game_state.time_of_day {
            time_changed_events.write(TimeChangedEvent {
                old_hour,
                new_hour: game_state.current_hour,
                old_time_of_day,
                new_time_of_day: game_state.time_of_day,
            });
        }

        // Update resources and send events for changes
        let resource_changes = [
            (ResourceType::Sleep, event.sleep_change),
            (ResourceType::Health, event.health_change),
            (ResourceType::Mental, event.mental_health_change),
            (ResourceType::Food, event.food_change),
        ];

        for (resource_type, change) in resource_changes {
            let old_value = game_state.get_resource_value(resource_type);
            let new_value = (old_value + change).clamp(0.0, 100.0);

            if (old_value - new_value).abs() > f32::EPSILON {
                match resource_type {
                    ResourceType::Sleep => game_state.sleep = new_value,
                    ResourceType::Health => game_state.health = new_value,
                    ResourceType::Mental => game_state.mental_health = new_value,
                    ResourceType::Food => game_state.food = new_value,
                }

                resource_changed_events.write(ResourceChangedEvent {
                    resource_type,
                    old_value,
                    new_value,
                });
            }
        }

        // Update mood and send event if changed
        let old_mood = game_state.current_mood;
        game_state.current_mood = game_state.calculate_mood();

        if old_mood != game_state.current_mood {
            mood_changed_events.write(MoodChangedEvent {
                old_mood,
                new_mood: game_state.current_mood,
            });
        }
    }
}

// Helper functions for other systems to create events
impl GameStepEvent {
    pub fn with_resource_change(mut self, resource_type: ResourceType, change: f32) -> Self {
        match resource_type {
            ResourceType::Sleep => self.sleep_change += change,
            ResourceType::Health => self.health_change += change,
            ResourceType::Mental => self.mental_health_change += change,
            ResourceType::Food => self.food_change += change,
        }
        self
    }
}

// Helper functions for phase management
impl GamePhaseState {
    pub fn get_phase_name(&self) -> &str {
        match self.current_phase {
            GamePhase::CardDraw => "Card Draw",
            GamePhase::CardSelection => "Card Selection",
            GamePhase::CharacterAction => "Character Action",
        }
    }

    pub fn is_cutscene_active(&self) -> bool {
        self.cutscene_active
    }
}
