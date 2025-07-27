use bevy::prelude::*;
use bevy_defer::{AsyncCommandsExtension, AsyncWorld};
use bevy_la_mesa::{events::DiscardCardToDeck, Card, CardOnTable, DeckArea, Hand};
use bevy_novel::{events::EventStartScenario, rpy_asset_loader::Rpy};
use std::collections::HashMap;

use crate::cards::*;
use crate::cutscene::ScenarioHandle;
use crate::navigation::GoToRandomTile;

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
            .add_event::<CardSelectionSuccess>()
            .add_event::<CardSelectionError>()
            .add_event::<ActionCompletedEvent>()
            .add_event::<TurnOverEvent>()
            .add_event::<CutsceneStartEvent>()
            .add_event::<CutsceneEndEvent>()
            .add_event::<StatusEffectAppliedEvent>()
            .add_event::<StatusEffectExpiredEvent>()
            .add_event::<CrisisLevelChangedEvent>()
            .add_event::<EndGameEvent>()
            .add_systems(
                Update,
                (
                    handle_game_step_events,
                    handle_phase_transitions,
                    handle_card_draw,
                    handle_card_selection,
                    handle_card_selection_success,
                    handle_character_action_phase,
                    handle_action_completion,
                    handle_turn_over,
                    handle_cutscene_trigger,
                    handle_cutscene_end,
                    handle_turn_over_completion,
                    handle_status_effect_tick,
                    handle_crisis_level_changes,
                    handle_end_game_check,
                    handle_daily_reset,
                )
                    .chain(),
            );
    }
}

#[derive(Event)]
pub struct StatusEffectAppliedEvent {
    pub effect: StatusEffect,
    pub duration: u32,
    pub source: String,
}

#[derive(Event)]
pub struct StatusEffectExpiredEvent {
    pub effect: StatusEffect,
}

#[derive(Event)]
pub struct CrisisLevelChangedEvent {
    pub old_level: CrisisLevel,
    pub new_level: CrisisLevel,
}

#[derive(Event)]
pub struct EndGameEvent {
    pub result: EndGameResult,
}

#[derive(Event)]
pub struct CardSelectedEvent(pub ActivityCard);

#[derive(Event)]
pub struct CardSelectionSuccess(pub ActivityCard);

#[derive(Event)]
pub struct CardSelectionError {
    pub card: ActivityCard,
    pub blocking_conditions: Vec<String>,
}

#[derive(Event)]
pub struct CutsceneEndEvent;

#[derive(Event)]
pub struct TurnOverEvent;

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
pub struct ActionCompletedEvent {
    pub card_played: ActivityCard,
}

#[derive(Event)]
pub struct CutsceneStartEvent {
    pub cutscene_id: String,
    pub trigger_reason: CutsceneTrigger,
}

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

// Game Data Structures
#[derive(Debug, Clone)]
pub struct ActiveStatusEffect {
    pub effect: StatusEffect,
    pub remaining_duration: u32,
    pub intensity: f32,
    pub source: String,
}

#[derive(Debug, Clone)]
pub struct PlayerAction {
    pub turn: u32,
    pub resources_before: (f32, f32, f32, f32),
    pub resources_after: (f32, f32, f32, f32),
    pub timestamp: f32,
}

#[derive(Debug, Clone)]
pub enum EndGameResult {
    Success(SuccessReason),
    Failure(FailureReason),
}

#[derive(Debug, Clone)]
pub enum SuccessReason {
    Stability,
    Recovery,
    Resilience,
    Growth,
}

#[derive(Debug, Clone)]
pub enum FailureReason {
    CompleteBreakdown,
    HealthCrisis,
    MentalCollapse,
    TimeLimit,
    CascadeFailure,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GamePhase {
    CardDraw,
    CardSelection,
    CharacterAction,
    TurnOver,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CutsceneTrigger {
    CardEffect,
    MoodChange,
    TimeOfDay,
    ResourceThreshold,
    TurnEnd,
}

// Game State
#[derive(Resource, Clone, Debug)]
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

    // Enhanced fields
    pub status_effects: Vec<ActiveStatusEffect>,
    pub locked_resources: HashMap<ResourceType, u32>,
    pub card_cooldowns: HashMap<u32, u32>,
    pub used_one_time_cards: Vec<u32>,
    pub daily_used_cards: Vec<u32>,
    pub crisis_level: CrisisLevel,
    pub consecutive_stable_days: u32,
    pub action_history: Vec<PlayerAction>,
    pub available_objects: Vec<String>,
    pub negative_card_count: u32,
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
            status_effects: Vec::new(),
            locked_resources: HashMap::new(),
            card_cooldowns: HashMap::new(),
            used_one_time_cards: Vec::new(),
            daily_used_cards: Vec::new(),
            crisis_level: CrisisLevel::None,
            consecutive_stable_days: 0,
            action_history: Vec::new(),
            available_objects: vec![
                "bed".to_string(),
                "kitchen".to_string(),
                "bathroom".to_string(),
                "tv".to_string(),
                "computer".to_string(),
                "phone".to_string(),
                "fridge".to_string(),
                "gameconsole".to_string(),
                "window".to_string(),
            ],
            negative_card_count: 0,
        }
    }
}

#[derive(Resource)]
pub struct GamePhaseState {
    pub current_phase: GamePhase,
    pub previous_phase: Option<GamePhase>,
    pub turn_number: u32,
    pub cards_drawn_count: usize,
    pub selected_card_id: Option<u32>,
    pub pending_cutscene: Option<String>,
    pub cutscene_active: bool,
}

impl Default for GamePhaseState {
    fn default() -> Self {
        Self {
            current_phase: GamePhase::CardDraw,
            previous_phase: None,
            turn_number: 1,
            cards_drawn_count: 0,
            selected_card_id: None,
            pending_cutscene: None,
            cutscene_active: false,
        }
    }
}

// Game State Implementation
impl GameState {
    pub fn can_play_card(&self, card: &ActivityCard) -> (bool, Vec<String>) {
        let mut blocking_conditions = Vec::new();

        // Check resource requirements
        if let Some(min_sleep) = card.conditions.min_sleep {
            if self.sleep < min_sleep {
                blocking_conditions.push(format!(
                    "Need at least {} Sleep (have {})",
                    min_sleep, self.sleep
                ));
            }
        }
        if let Some(max_sleep) = card.conditions.max_sleep {
            if self.sleep > max_sleep {
                blocking_conditions.push(format!("Sleep too high (max {})", max_sleep));
            }
        }
        if let Some(min_health) = card.conditions.min_health {
            if self.health < min_health {
                blocking_conditions.push(format!("Need at least {} Health", min_health));
            }
        }
        if let Some(max_health) = card.conditions.max_health {
            if self.health > max_health {
                blocking_conditions.push(format!("Health too high (max {})", max_health));
            }
        }
        if let Some(min_mental) = card.conditions.min_mental {
            if self.mental_health < min_mental {
                blocking_conditions.push(format!("Need at least {} Mental Health", min_mental));
            }
        }
        if let Some(max_mental) = card.conditions.max_mental {
            if self.mental_health > max_mental {
                blocking_conditions.push(format!("Mental Health too high (max {})", max_mental));
            }
        }
        if let Some(min_food) = card.conditions.min_food {
            if self.food < min_food {
                blocking_conditions.push(format!("Need at least {} Food", min_food));
            }
        }
        if let Some(max_food) = card.conditions.max_food {
            if self.food > max_food {
                blocking_conditions.push(format!("Food too high (max {})", max_food));
            }
        }

        // Check mood requirements
        if let Some(required_mood) = &card.conditions.required_mood {
            if self.current_mood != *required_mood {
                blocking_conditions.push(format!("Must be {:?} mood", required_mood));
            }
        }
        if let Some(forbidden_mood) = &card.conditions.forbidden_mood {
            if self.current_mood == *forbidden_mood {
                blocking_conditions.push(format!("Cannot be {:?} mood", forbidden_mood));
            }
        }

        // Check time requirements
        if let Some(allowed_times) = &card.conditions.time_of_day {
            if !allowed_times.contains(&self.time_of_day) {
                blocking_conditions.push(format!("Wrong time of day (need {:?})", allowed_times));
            }
        }

        // Check day range
        if let Some((min_day, max_day)) = card.conditions.day_range {
            if self.current_day < min_day || self.current_day > max_day {
                blocking_conditions.push(format!("Wrong day (need days {}-{})", min_day, max_day));
            }
        }

        // Check required objects
        if let Some(required_objects) = card.conditions.required_objects.clone() {
            for required_object in &required_objects {
                if !self.available_objects.contains(required_object) {
                    blocking_conditions.push(format!("Need {}", required_object));
                }
            }
        }

        // Check crisis level
        if let Some(required_crisis) = &card.conditions.crisis_level {
            if self.crisis_level != *required_crisis {
                blocking_conditions.push(format!("Need {:?} crisis level", required_crisis));
            }
        }

        // Check cooldowns
        if let Some(cooldown) = card.cooldown {
            if let Some(&last_used_turn) = self.card_cooldowns.get(&card.id) {
                let turns_since = self.turn_number() - last_used_turn;
                if turns_since < cooldown {
                    blocking_conditions.push(format!(
                        "On cooldown for {} more turns",
                        cooldown - turns_since
                    ));
                }
            }
        }

        // Check one-time use
        if card.one_time_use && self.used_one_time_cards.contains(&card.id) {
            blocking_conditions.push("Already used (one-time only)".to_string());
        }

        // Check daily use
        if matches!(card.availability, CardAvailability::DailyReset)
            && self.daily_used_cards.contains(&card.id)
        {
            blocking_conditions.push("Already used today".to_string());
        }

        // Check costs
        if !self.can_afford_costs(&card.costs) {
            if self.sleep < card.costs.sleep_cost {
                blocking_conditions.push(format!(
                    "Not enough Sleep ({} needed)",
                    card.costs.sleep_cost
                ));
            }
            if self.health < card.costs.health_cost {
                blocking_conditions.push(format!(
                    "Not enough Health ({} needed)",
                    card.costs.health_cost
                ));
            }
            if self.mental_health < card.costs.mental_cost {
                blocking_conditions.push(format!(
                    "Not enough Mental ({} needed)",
                    card.costs.mental_cost
                ));
            }
            if self.food < card.costs.food_cost {
                blocking_conditions
                    .push(format!("Not enough Food ({} needed)", card.costs.food_cost));
            }
        }

        (blocking_conditions.is_empty(), blocking_conditions)
    }

    pub fn can_afford_costs(&self, costs: &CardCosts) -> bool {
        self.sleep >= costs.sleep_cost
            && self.health >= costs.health_cost
            && self.mental_health >= costs.mental_cost
            && self.food >= costs.food_cost
    }

    pub fn calculate_crisis_level(&self) -> CrisisLevel {
        let resources_below_30 = [self.sleep, self.health, self.mental_health, self.food]
            .iter()
            .filter(|&&r| r < 30.0)
            .count();

        let resources_below_15 = [self.sleep, self.health, self.mental_health, self.food]
            .iter()
            .filter(|&&r| r < 15.0)
            .count();

        let resources_below_10 = [self.sleep, self.health, self.mental_health, self.food]
            .iter()
            .filter(|&&r| r < 10.0)
            .count();

        match (resources_below_10, resources_below_15, resources_below_30) {
            (2.., _, _) => CrisisLevel::Critical,
            (1, _, _) => CrisisLevel::Severe,
            (0, 1, _) => CrisisLevel::Moderate,
            (0, 0, 1..) => CrisisLevel::Mild,
            _ => CrisisLevel::None,
        }
    }

    pub fn apply_card_effects(&mut self, card: &ActivityCard) -> Vec<StatusEffectAppliedEvent> {
        let mut status_events = Vec::new();

        // Store resources before for history
        let resources_before = (self.sleep, self.health, self.mental_health, self.food);

        // Pay costs
        self.sleep = (self.sleep - card.costs.sleep_cost).max(0.0);
        self.health = (self.health - card.costs.health_cost).max(0.0);
        self.mental_health = (self.mental_health - card.costs.mental_cost).max(0.0);
        self.food = (self.food - card.costs.food_cost).max(0.0);

        // Apply effects with multipliers
        let effect_multiplier = self.calculate_effect_multiplier();

        self.sleep = (self.sleep + card.effects.sleep * effect_multiplier).clamp(0.0, 100.0);
        self.health = (self.health + card.effects.health * effect_multiplier).clamp(0.0, 100.0);
        self.mental_health =
            (self.mental_health + card.effects.mental * effect_multiplier).clamp(0.0, 100.0);
        self.food = (self.food + card.effects.food * effect_multiplier).clamp(0.0, 100.0);

        // Apply status effects
        for status_app in &card.status_effects {
            self.add_status_effect(status_app.effect.clone(), status_app.duration, &card.name);
            status_events.push(StatusEffectAppliedEvent {
                effect: status_app.effect.clone(),
                duration: status_app.duration,
                source: card.name.clone(),
            });
        }

        // Handle additional costs
        for additional_cost in &card.costs.additional_costs {
            match additional_cost.as_str() {
                s if s.starts_with("AddNegativeCard") => {
                    self.negative_card_count += 1;
                }
                s if s.starts_with("LockResource") => {
                    self.locked_resources.insert(ResourceType::Mental, 2);
                }
                s if s.starts_with("ConsumeAllOfResource") => {
                    if s.contains("Mental") {
                        self.mental_health = 0.0;
                    } else if s.contains("Sleep") {
                        self.sleep = 0.0;
                    }
                }
                _ => {}
            }
        }

        // Update cooldown
        if let Some(cooldown) = card.cooldown {
            self.card_cooldowns.insert(card.id, self.turn_number());
        }

        // Mark usage
        if card.one_time_use {
            self.used_one_time_cards.push(card.id);
        }

        if matches!(card.availability, CardAvailability::DailyReset) {
            self.daily_used_cards.push(card.id);
        }

        // Update crisis level
        self.crisis_level = self.calculate_crisis_level();

        // Add to action history
        let resources_after = (self.sleep, self.health, self.mental_health, self.food);
        self.action_history.push(PlayerAction {
            turn: self.turn_number(),
            resources_before,
            resources_after,
            timestamp: self.current_hour,
        });

        status_events
    }

    fn calculate_effect_multiplier(&self) -> f32 {
        let mut multiplier = 1.0;

        for status in &self.status_effects {
            match &status.effect {
                StatusEffect::Motivated(_) => multiplier += 0.5 * status.intensity,
                StatusEffect::Depressed(_) => multiplier -= 0.3 * status.intensity,
                StatusEffect::Overwhelmed(_) => multiplier -= 0.2 * status.intensity,
                StatusEffect::Focused(_) => multiplier += 0.3 * status.intensity,
                _ => {}
            }
        }

        multiplier.max(0.1)
    }

    pub fn add_status_effect(&mut self, effect: StatusEffect, duration: u32, source: &str) {
        self.status_effects.push(ActiveStatusEffect {
            effect,
            remaining_duration: duration,
            intensity: 1.0,
            source: source.to_string(),
        });
    }

    pub fn tick_status_effects(&mut self) -> Vec<StatusEffectExpiredEvent> {
        let mut expired_events = Vec::new();

        let mut i = 0;
        while i < self.status_effects.len() {
            self.status_effects[i].remaining_duration -= 1;
            if self.status_effects[i].remaining_duration == 0 {
                let expired = self.status_effects.remove(i);
                expired_events.push(StatusEffectExpiredEvent {
                    effect: expired.effect,
                });
            } else {
                i += 1;
            }
        }

        for duration in self.locked_resources.values_mut() {
            *duration = duration.saturating_sub(1);
        }
        self.locked_resources
            .retain(|_, &mut duration| duration > 0);

        expired_events
    }

    pub fn check_end_game_conditions(&self) -> Option<EndGameResult> {
        // Failure conditions
        let all_resources_critical = [self.sleep, self.health, self.mental_health, self.food]
            .iter()
            .all(|&r| r < 5.0);

        if all_resources_critical {
            return Some(EndGameResult::Failure(FailureReason::CompleteBreakdown));
        }

        if self.health <= 0.0 {
            return Some(EndGameResult::Failure(FailureReason::HealthCrisis));
        }

        if self.mental_health <= 0.0 && self.has_depression_spiral() {
            return Some(EndGameResult::Failure(FailureReason::MentalCollapse));
        }

        if self.current_day >= 30 && self.crisis_level == CrisisLevel::Critical {
            return Some(EndGameResult::Failure(FailureReason::TimeLimit));
        }

        let negative_status_count = self
            .status_effects
            .iter()
            .filter(|s| {
                matches!(
                    s.effect,
                    StatusEffect::Depressed(_)
                        | StatusEffect::Anxious(_)
                        | StatusEffect::Overwhelmed(_)
                )
            })
            .count();
        if negative_status_count >= 3 {
            return Some(EndGameResult::Failure(FailureReason::CascadeFailure));
        }

        // Success conditions
        if self.consecutive_stable_days >= 7 {
            return Some(EndGameResult::Success(SuccessReason::Stability));
        }

        let all_resources_high = [self.sleep, self.health, self.mental_health, self.food]
            .iter()
            .all(|&r| r >= 80.0);

        if all_resources_high {
            return Some(EndGameResult::Success(SuccessReason::Recovery));
        }

        if self.current_day >= 30 && self.crisis_level == CrisisLevel::None {
            return Some(EndGameResult::Success(SuccessReason::Resilience));
        }

        None
    }

    fn has_depression_spiral(&self) -> bool {
        self.status_effects.iter().any(|status| {
            matches!(status.effect, StatusEffect::Depressed(_)) && status.remaining_duration > 5
        })
    }

    pub fn turn_number(&self) -> u32 {
        self.action_history.len() as u32 + 1
    }

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

// Helper functions for phase management
impl GamePhaseState {
    pub fn get_phase_name(&self) -> &str {
        match self.current_phase {
            GamePhase::CardDraw => "Card Draw",
            GamePhase::CardSelection => "Card Selection",
            GamePhase::CharacterAction => "Character Action",
            GamePhase::TurnOver => "Turn Over",
        }
    }

    pub fn is_cutscene_active(&self) -> bool {
        self.cutscene_active
    }
}

// System implementations
fn handle_card_draw(
    mut card_drawn_events: EventReader<CardDrawnEvent>,
    mut phase_state: ResMut<GamePhaseState>,
    mut phase_changed_events: EventWriter<PhaseChangedEvent>,
) {
    for event in card_drawn_events.read() {
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

fn handle_card_selection(
    mut card_selected_events: EventReader<CardSelectedEvent>,
    mut card_selection_success_events: EventWriter<CardSelectionSuccess>,
    mut card_selection_error_events: EventWriter<CardSelectionError>,
    phase_state: Res<GamePhaseState>,
    game_state: Res<GameState>,
) {
    for event in card_selected_events.read() {
        if phase_state.cutscene_active {
            continue;
        }

        let (can_play, blocking_conditions) = game_state.can_play_card(&event.0);

        println!(
            "can play: {}, condition: {:?}",
            can_play, blocking_conditions
        );

        if !can_play {
            info!("Cannot play card: {:?}", blocking_conditions);
            card_selection_error_events.write(CardSelectionError {
                card: event.0.clone(),
                blocking_conditions,
            });
            continue;
        }

        // Emit success event when card can be played
        card_selection_success_events.write(CardSelectionSuccess(event.0.clone()));
    }
}

fn handle_card_selection_success(
    mut card_selection_success_events: EventReader<CardSelectionSuccess>,
    mut phase_state: ResMut<GamePhaseState>,
    mut phase_changed_events: EventWriter<PhaseChangedEvent>,
    mut cutscene_events: EventWriter<CutsceneStartEvent>,
    mut status_effect_events: EventWriter<StatusEffectAppliedEvent>,
    mut game_state: ResMut<GameState>,
) {
    for event in card_selection_success_events.read() {
        if phase_state.cutscene_active {
            continue;
        }

        phase_state.selected_card_id = Some(event.0.id);

        // Check if cutscene should be triggered based on card type
        let should_trigger_cutscene =
            matches!(event.0.card_type, CardType::Crisis | CardType::ComboCard);

        let status_events = game_state.apply_card_effects(&event.0);
        for status_event in status_events {
            status_effect_events.write(status_event);
        }

        if should_trigger_cutscene {
            cutscene_events.write(CutsceneStartEvent {
                cutscene_id: format!("card_{}", event.0.id),
                trigger_reason: CutsceneTrigger::CardEffect,
            });
        } else {
            // Go to CharacterAction phase
            let old_phase = phase_state.current_phase;
            phase_state.current_phase = GamePhase::CharacterAction;

            phase_changed_events.write(PhaseChangedEvent {
                old_phase,
                new_phase: phase_state.current_phase,
            });
        }
    }
}

fn handle_status_effect_tick(
    mut game_state: ResMut<GameState>,
    mut status_expired_events: EventWriter<StatusEffectExpiredEvent>,
    mut phase_changed_events: EventReader<PhaseChangedEvent>,
) {
    for event in phase_changed_events.read() {
        if event.new_phase == GamePhase::TurnOver {
            let expired_events = game_state.tick_status_effects();

            for expired_event in expired_events {
                status_expired_events.write(expired_event);
            }
        }
    }
}

fn handle_crisis_level_changes(
    mut game_state: ResMut<GameState>,
    mut crisis_changed_events: EventWriter<CrisisLevelChangedEvent>,
    mut resource_changed_events: EventReader<ResourceChangedEvent>,
) {
    for _event in resource_changed_events.read() {
        let old_level = game_state.crisis_level.clone();
        let new_level = game_state.calculate_crisis_level();

        if old_level != new_level {
            game_state.crisis_level = new_level.clone();
            crisis_changed_events.write(CrisisLevelChangedEvent {
                old_level,
                new_level,
            });
        }
    }
}

fn handle_end_game_check(
    game_state: Res<GameState>,
    mut end_game_events: EventWriter<EndGameEvent>,
    mut resource_changed_events: EventReader<ResourceChangedEvent>,
) {
    for _event in resource_changed_events.read() {
        if let Some(result) = game_state.check_end_game_conditions() {
            end_game_events.write(EndGameEvent { result });
        }
    }
}

fn handle_daily_reset(
    mut day_changed_events: EventReader<DayChangedEvent>,
    mut game_state: ResMut<GameState>,
) {
    for _event in day_changed_events.read() {
        // Reset daily used cards
        game_state.daily_used_cards.clear();

        // Check for consecutive stable days
        let all_resources_stable = [
            game_state.sleep,
            game_state.health,
            game_state.mental_health,
            game_state.food,
        ]
        .iter()
        .all(|&r| r >= 50.0);

        if all_resources_stable && game_state.crisis_level == CrisisLevel::None {
            game_state.consecutive_stable_days += 1;
        } else {
            game_state.consecutive_stable_days = 0;
        }

        info!(
            "New day! Consecutive stable days: {}",
            game_state.consecutive_stable_days
        );
    }
}

fn handle_phase_transitions(
    mut phase_state: ResMut<GamePhaseState>,
    mut phase_changed_events: EventReader<PhaseChangedEvent>,
) {
    for event in phase_changed_events.read() {
        phase_state.current_phase = event.new_phase.clone();
    }
}

fn handle_cutscene_trigger(
    mut cutscene_events: EventReader<CutsceneStartEvent>,
    mut phase_state: ResMut<GamePhaseState>,
    mut novel_events: EventWriter<EventStartScenario>,
    scenario: Res<ScenarioHandle>,
    rpy_assets: Res<Assets<Rpy>>,
) {
    for event in cutscene_events.read() {
        phase_state.pending_cutscene = Some(event.cutscene_id.clone());
        phase_state.previous_phase = Some(phase_state.current_phase);
        phase_state.cutscene_active = true;

        info!(
            "Starting cutscene: {} during {:?} phase",
            event.cutscene_id, phase_state.current_phase
        );

        if let Some(rpy) = rpy_assets.get(scenario.id()) {
            novel_events.write(EventStartScenario { ast: rpy.0.clone() });
        }
    }
}

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

        phase_state.cutscene_active = false;
        phase_state.pending_cutscene = None;

        // Resume from the phase where cutscene was triggered
        if let Some(previous_phase) = phase_state.previous_phase {
            match previous_phase {
                GamePhase::CardSelection => {
                    // Card was selected, go to CharacterAction
                    let old_phase = phase_state.current_phase;
                    phase_state.current_phase = GamePhase::CharacterAction;

                    phase_changed_events.write(PhaseChangedEvent {
                        old_phase,
                        new_phase: phase_state.current_phase,
                    });
                }
                GamePhase::CharacterAction => {
                    // Character action was in progress, complete it
                    action_completed_events.write(ActionCompletedEvent {
                        card_played: ActivityCard::default(), // You may need to store the card being played
                    });
                }
                _ => {}
            }
        }

        phase_state.previous_phase = None;
    }
}

fn handle_character_action_phase(
    mut commands: Commands,
    mut phase_changed_events: EventReader<PhaseChangedEvent>,
    phase_state: Res<GamePhaseState>,
) {
    for event in phase_changed_events.read() {
        if event.new_phase == GamePhase::CharacterAction {
            if let Some(_) = phase_state.selected_card_id {
                commands.spawn_task(move || async move {
                    AsyncWorld.sleep(3.0).await;
                    AsyncWorld.send_event(GoToRandomTile {})?;

                    Ok(())
                });
            }
        }
    }
}

fn handle_action_completion(
    mut action_completed_events: EventReader<ActionCompletedEvent>,
    mut phase_state: ResMut<GamePhaseState>,
    mut phase_changed_events: EventWriter<PhaseChangedEvent>,
    mut game_step_events: EventWriter<GameStepEvent>,
) {
    for event in action_completed_events.read() {
        if phase_state.cutscene_active {
            continue;
        }

        info!("Action completed, transitioning to TurnOver phase");

        // Apply time cost from played card
        game_step_events.write(GameStepEvent {
            time_delta: event.card_played.costs.time_cost * 3600.0, // Convert hours to seconds
            sleep_change: 0.0,
            health_change: 0.0,
            mental_health_change: 0.0,
            food_change: 0.0,
        });

        let old_phase = phase_state.current_phase;
        phase_state.current_phase = GamePhase::TurnOver;

        phase_changed_events.write(PhaseChangedEvent {
            old_phase,
            new_phase: phase_state.current_phase,
        });
    }
}

fn handle_turn_over(
    mut phase_changed_events: EventReader<PhaseChangedEvent>,
    mut turn_over_events: EventWriter<TurnOverEvent>,
    mut cutscene_events: EventWriter<CutsceneStartEvent>,
    phase_state: Res<GamePhaseState>,
    game_state: Res<GameState>,
) {
    for event in phase_changed_events.read() {
        if event.new_phase == GamePhase::TurnOver {
            info!(
                "Entered TurnOver phase for turn {}",
                phase_state.turn_number
            );

            let should_trigger_cutscene =
                check_turn_end_cutscene_triggers(&game_state, &phase_state);

            if let Some(cutscene_id) = should_trigger_cutscene {
                cutscene_events.write(CutsceneStartEvent {
                    cutscene_id,
                    trigger_reason: CutsceneTrigger::TurnEnd,
                });
            } else {
                turn_over_events.write(TurnOverEvent);
            }
        }
    }
}

fn check_turn_end_cutscene_triggers(
    game_state: &GameState,
    phase_state: &GamePhaseState,
) -> Option<String> {
    // Check for critical resource levels
    if game_state.health < 10.0 {
        return Some("critical_health".to_string());
    }

    if game_state.mental_health < 10.0 {
        return Some("mental_breakdown".to_string());
    }

    // Check for special days or turn milestones
    if phase_state.turn_number % 10 == 0 {
        return Some("turn_milestone".to_string());
    }

    // Check for time-based triggers
    if game_state.current_hour >= 22.0 && game_state.time_of_day == TimeOfDay::Night {
        return Some("late_night_reflection".to_string());
    }

    // Check for mood-based triggers
    match game_state.current_mood {
        Mood::Depressed => Some("depression_cutscene".to_string()),
        Mood::Manic => Some("manic_episode".to_string()),
        _ => None,
    }
}

fn handle_turn_over_completion(
    mut commands: Commands,
    mut turn_over_events: EventReader<TurnOverEvent>,
    mut phase_state: ResMut<GamePhaseState>,
    mut ew_go_to_random_tile: EventWriter<GoToRandomTile>,
    mut game_step_events: EventWriter<GameStepEvent>,
    q_decks: Query<(Entity, &DeckArea)>,
    mut q_cards: ParamSet<(
        Query<(Entity, &Card<ActivityCard>, &CardOnTable)>,
        Query<(Entity, &Card<ActivityCard>, &Hand)>,
    )>,
    mut ew_discard_card_to_deck: EventWriter<DiscardCardToDeck>,
) {
    for _event in turn_over_events.read() {
        if phase_state.cutscene_active {
            continue;
        }

        if let Some((main_deck_entity, _)) = q_decks.iter().find(|(_, deck)| deck.marker == 1) {
            // Discard cards from table
            for (entity, _, _) in q_cards.p0().iter() {
                ew_discard_card_to_deck.write(DiscardCardToDeck {
                    card_entity: entity,
                    deck_entity: main_deck_entity,
                    flip_card: true,
                });
            }

            // Discard cards from hand
            for (entity, _, _) in q_cards.p1().iter() {
                ew_discard_card_to_deck.write(DiscardCardToDeck {
                    card_entity: entity,
                    deck_entity: main_deck_entity,
                    flip_card: true,
                });
            }
        }

        info!("Processing turn over, starting new turn");

        // Clear phase state from previous turn
        phase_state.selected_card_id = None;
        phase_state.cards_drawn_count = 0;

        // Increment turn number
        phase_state.turn_number += 1;

        // Apply passive effects (time passage, resource decay, etc.)
        apply_turn_end_effects(&mut game_step_events);

        let old_phase = phase_state.current_phase;

        ew_go_to_random_tile.write(GoToRandomTile {});

        commands.spawn_task(move || async move {
            AsyncWorld.sleep(2.5).await;

            // Transition to card draw for new turn
            AsyncWorld.send_event(PhaseChangedEvent {
                old_phase,
                new_phase: GamePhase::CardDraw,
            })?;

            Ok(())
        });

        info!("Started turn {}", phase_state.turn_number);
    }
}

fn apply_turn_end_effects(game_step_events: &mut EventWriter<GameStepEvent>) {
    // Apply passive time and resource changes
    let time_passage = 3600.0; // 1 hour per turn
    let base_resource_decay = -5.0;

    game_step_events.write(GameStepEvent {
        time_delta: time_passage,
        sleep_change: base_resource_decay,
        health_change: base_resource_decay * 0.5,
        mental_health_change: base_resource_decay * 0.8,
        food_change: base_resource_decay * 1.2,
    });
}

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
