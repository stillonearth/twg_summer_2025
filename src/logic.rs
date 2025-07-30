use bevy::prelude::*;
use bevy_defer::{AsyncCommandsExtension, AsyncWorld};
use bevy_la_mesa::{Card, Hand};
use bevy_novel::{events::EventStartScenario, rpy_asset_loader::Rpy};
use rand::{Rng, rng};
use std::collections::HashMap;

use crate::cutscene::ScenarioHandle;
use crate::game_objects::NavigateToObjectEvent;
use crate::navigation::GoToRandomTile;
use crate::{AppState, cards::*};

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
            // New adversary events
            .add_event::<AdversaryCardDrawnEvent>()
            .add_event::<AdversaryCardSelectedEvent>()
            .add_event::<AdversaryActionCompletedEvent>()
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
                    handle_adversary_card_selection_phase,
                    handle_adversary_card_draw,
                    handle_adversary_card_selection,
                    handle_adversary_action_completion,
                    handle_phase_changed_turn_over,
                    handle_turn_over,
                    handle_cutscene_trigger,
                    handle_cutscene_end,
                    handle_status_effect_tick,
                    handle_crisis_level_changes,
                    handle_end_game_check,
                    handle_daily_reset,
                )
                    .run_if(in_state(AppState::Game)),
            );
    }
}

// New adversary events
#[derive(Event)]
pub struct AdversaryCardDrawnEvent {
    pub card_count: usize,
}

#[derive(Event)]
pub struct AdversaryCardSelectedEvent {
    pub card: SchizophrenicCard,
}

#[derive(Event)]
pub struct AdversaryActionCompletedEvent {
    pub card_played: SchizophrenicCard,
}

// Enhanced GamePhase enum with new adversary phases
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GamePhase {
    CardDraw,
    CardSelection,
    CharacterAction,
    AdversaryCardDraw,
    AdversaryCardSelection,
    TurnOver,
}

#[derive(Debug, Clone)]
pub struct AdversaryEffects {
    pub player_sleep_change: f32,
    pub player_health_change: f32,
    pub player_mental_change: f32,
    pub player_food_change: f32,
    pub status_effects: Vec<StatusEffectApplication>,
    pub environmental_changes: Vec<EnvironmentalChange>,
}

#[derive(Debug, Clone)]
pub struct AdversaryConditions {
    pub min_crisis_level: Option<CrisisLevel>,
    pub max_crisis_level: Option<CrisisLevel>,
    pub required_time_of_day: Option<Vec<TimeOfDay>>,
    pub player_resource_thresholds: Option<ResourceThresholds>,
    pub turn_number_range: Option<(u32, u32)>,
}

#[derive(Debug, Clone)]
pub struct ResourceThresholds {
    pub min_sleep: Option<f32>,
    pub max_sleep: Option<f32>,
    pub min_health: Option<f32>,
    pub max_health: Option<f32>,
    pub min_mental: Option<f32>,
    pub max_mental: Option<f32>,
    pub min_food: Option<f32>,
    pub max_food: Option<f32>,
}

#[derive(Debug, Clone)]
pub enum AdversaryCardType {
    Crisis,        // Negative events that challenge the player
    Environmental, // Changes to the game environment
    Social,        // Social pressures or interactions
    Random,        // Random events
    Consequence,   // Direct consequences of player actions
}

#[derive(Debug, Clone)]
pub enum EnvironmentalChange {
    RemoveObject(String),
    AddObject(String),
    LockResource(ResourceType, u32), // Resource type and duration
    WeatherChange(WeatherType),
}

#[derive(Debug, Clone)]
pub enum WeatherType {
    Sunny,
    Rainy,
    Stormy,
    Cloudy,
}

// Rest of the existing events and structures remain the same...
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
pub struct CardSelectionError {
    pub card: ActivityCard,
    pub blocking_conditions: Vec<String>,
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
pub struct CutsceneEndEvent;

#[derive(Event)]
pub struct TurnOverEvent;

#[derive(Event)]
pub struct PhaseChangedEvent {
    pub new_phase: GamePhase,
}

#[derive(Event)]
pub struct CardDrawnEvent {
    pub card_count: usize,
}

#[derive(Event)]
pub struct ActionCompletedEvent {
    pub card_played: GameCard,
}

#[derive(Event)]
pub struct CutsceneStartEvent {
    pub cutscene_id: String,
    pub card_id: Option<u32>,
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

// Game Data Structures (existing ones remain the same)
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

#[derive(Debug, Clone, PartialEq)]
pub enum CutsceneTrigger {
    CardEffect,
    MoodChange,
    TimeOfDay,
    ResourceThreshold,
    TurnEnd,
    AdversaryAction, // New trigger type
}

// Enhanced GamePhaseState with adversary state
#[derive(Resource)]
pub struct GamePhaseState {
    pub current_phase: GamePhase,
    pub previous_phase: Option<GamePhase>,
    pub turn_number: u32,
    pub cards_drawn_count: usize,
    pub selected_card_id: Option<u32>,
    pub pending_cutscene: Option<String>,
    pub cutscene_active: bool,
    pub pending_post_action_cutscene: Option<PendingCutscene>,
    // New adversary fields
    pub adversary_cards_drawn: Vec<SchizophrenicCard>,
    pub selected_adversary_card: Option<SchizophrenicCard>,
    pub adversary_deck: Vec<SchizophrenicCard>, // Available adversary cards
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
            pending_post_action_cutscene: None,
            adversary_cards_drawn: Vec::new(),
            selected_adversary_card: None,
            adversary_deck: Vec::new(), // Will be loaded from JSON
        }
    }
}

// Enhanced GameState remains mostly the same but with new fields
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
    pub shown_cutscenes: std::collections::HashSet<u32>,

    // New adversary-related fields
    pub current_weather: WeatherType,
    pub adversary_action_history: Vec<AdversaryAction>,
}

#[derive(Debug, Clone)]
pub struct AdversaryAction {
    pub turn: u32,
    pub card_played: SchizophrenicCard,
    pub effects_applied: AdversaryEffects,
    pub timestamp: f32,
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
            shown_cutscenes: std::collections::HashSet::new(),
            current_weather: WeatherType::Sunny,
            adversary_action_history: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct PendingCutscene {
    pub cutscene_id: String,
    pub card_id: Option<u32>,
    pub trigger_reason: CutsceneTrigger,
}

// Implementation methods for adversary logic
impl GameState {
    // Existing methods remain the same...
    pub fn can_play_card(&self, card: &ActivityCard) -> (bool, Vec<String>) {
        let mut blocking_conditions = Vec::new();

        // Check resource requirements
        if let Some(min_sleep) = card.conditions.min_sleep
            && self.sleep < min_sleep
        {
            blocking_conditions.push(format!(
                "Need at least {} Sleep (have {})",
                min_sleep, self.sleep
            ));
        }
        if let Some(max_sleep) = card.conditions.max_sleep
            && self.sleep > max_sleep
        {
            blocking_conditions.push(format!("Sleep too high (max {max_sleep})"));
        }
        if let Some(min_health) = card.conditions.min_health
            && self.health < min_health
        {
            blocking_conditions.push(format!("Need at least {min_health} Health"));
        }
        if let Some(max_health) = card.conditions.max_health
            && self.health > max_health
        {
            blocking_conditions.push(format!("Health too high (max {max_health})"));
        }
        if let Some(min_mental) = card.conditions.min_mental
            && self.mental_health < min_mental
        {
            blocking_conditions.push(format!("Need at least {min_mental} Mental Health"));
        }
        if let Some(max_mental) = card.conditions.max_mental
            && self.mental_health > max_mental
        {
            blocking_conditions.push(format!("Mental Health too high (max {max_mental})"));
        }
        if let Some(min_food) = card.conditions.min_food
            && self.food < min_food
        {
            blocking_conditions.push(format!("Need at least {min_food} Food"));
        }
        if let Some(max_food) = card.conditions.max_food
            && self.food > max_food
        {
            blocking_conditions.push(format!("Food too high (max {max_food})"));
        }

        // Check mood requirements
        if let Some(required_mood) = &card.conditions.required_mood
            && self.current_mood != *required_mood
        {
            blocking_conditions.push(format!("Must be {required_mood:?} mood"));
        }
        if let Some(forbidden_mood) = &card.conditions.forbidden_mood
            && self.current_mood == *forbidden_mood
        {
            blocking_conditions.push(format!("Cannot be {forbidden_mood:?} mood"));
        }

        // Check time requirements
        if let Some(allowed_times) = &card.conditions.time_of_day
            && !allowed_times.contains(&self.time_of_day)
        {
            blocking_conditions.push(format!("Wrong time of day (need {allowed_times:?})"));
        }

        // Check day range
        if let Some((min_day, max_day)) = card.conditions.day_range
            && (self.current_day < min_day || self.current_day > max_day)
        {
            blocking_conditions.push(format!("Wrong day (need days {min_day}-{max_day})"));
        }

        // Check required objects
        if let Some(required_objects) = card.conditions.required_objects.clone() {
            for required_object in &required_objects {
                if !self.available_objects.contains(required_object) {
                    blocking_conditions.push(format!("Need {required_object}"));
                }
            }
        }

        // Check crisis level
        if let Some(required_crisis) = &card.conditions.crisis_level
            && self.crisis_level != *required_crisis
        {
            blocking_conditions.push(format!("Need {required_crisis:?} crisis level"));
        }

        // Check cooldowns
        if let Some(cooldown) = card.cooldown
            && let Some(&last_used_turn) = self.card_cooldowns.get(&card.id)
        {
            let turns_since = self.turn_number() - last_used_turn;
            if turns_since < cooldown {
                blocking_conditions.push(format!(
                    "On cooldown for {} more turns",
                    cooldown - turns_since
                ));
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

    pub fn filter_cards(&self, cards: &[GameCard]) -> Vec<GameCard> {
        cards
            .iter()
            .filter(|card| {
                match &card.card_variant {
                    CardVariant::Activity(activity_card) => {
                        let (can_play, _) = self.can_play_card(activity_card);
                        can_play
                    }
                    CardVariant::Schizophrenic(_) => false, // Don't include schizophrenic cards in activity filtering
                }
            })
            .cloned()
            .collect()
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
            GamePhase::AdversaryCardDraw => "Adversary Card Draw",
            GamePhase::AdversaryCardSelection => "Adversary Card Selection",
            GamePhase::TurnOver => "Turn Over",
        }
    }

    pub fn is_cutscene_active(&self) -> bool {
        self.cutscene_active
    }
}

fn handle_adversary_card_selection_phase(
    mut phase_changed_events: EventReader<PhaseChangedEvent>,
    mut adversary_card_selected_events: EventWriter<AdversaryCardSelectedEvent>,
    mut phase_state: ResMut<GamePhaseState>,
    q_cards: Query<(Entity, &Card<GameCard>, &Hand)>,
) {
    for event in phase_changed_events.read() {
        if event.new_phase == GamePhase::AdversaryCardSelection {
            info!("Entering Adversary Card Selection phase");

            // Get all schizophrenic cards in adversary's hand (player 2)
            let adversary_cards: Vec<_> = q_cards
                .iter()
                .filter(|(_, card, hand)| {
                    hand.player == 2
                        && matches!(card.data.card_variant, CardVariant::Schizophrenic(_))
                })
                .collect();

            if adversary_cards.len() == 0 {
                continue;
            }

            // Select a random card from adversary's hand
            let mut rng = rand::thread_rng();
            let random_index: usize = rng.gen_range(0..adversary_cards.len());
            let (_, selected_card, _) = &adversary_cards[random_index];

            // Extract the schizophrenic card data
            if let Some(schizo_card) = selected_card.data.to_schizophrenic_card() {
                info!("Adversary selected card: {}", schizo_card.card_name);

                adversary_card_selected_events.write(AdversaryCardSelectedEvent {
                    card: schizo_card.clone(),
                });
            }
        }
    }
}

fn handle_adversary_card_draw(
    mut commands: Commands,
    mut adversary_card_drawn_events: EventReader<AdversaryCardDrawnEvent>,
) {
    for _ in adversary_card_drawn_events.read() {
        // Automatically proceed to adversary card selection
        commands.spawn_task(move || async move {
            AsyncWorld.sleep(3.0).await;
            AsyncWorld.send_event(PhaseChangedEvent {
                new_phase: GamePhase::AdversaryCardSelection,
            })?;
            Ok(())
        });
    }
}

fn handle_adversary_card_selection(
    mut adversary_card_selected_events: EventReader<AdversaryCardSelectedEvent>,
    mut adversary_action_completed_events: EventWriter<AdversaryActionCompletedEvent>,
    mut status_effect_events: EventWriter<StatusEffectAppliedEvent>,
    mut game_state: ResMut<GameState>,
    mut ew_drag_cards: EventWriter<DragCardsInHandUp>,
) {
    for event in adversary_card_selected_events.read() {
        info!("Adversary selected card: {}", event.card.title);

        // Apply adversary card effects
        // let status_events = game_state.apply_adversary_card_effects(&event.card);
        // for status_event in status_events {
        //     status_effect_events.write(status_event);
        // }

        // Signal that adversary action is complete
        adversary_action_completed_events.write(AdversaryActionCompletedEvent {
            card_played: event.card.clone(),
        });

        ew_drag_cards.write(DragCardsInHandUp { player: 2 });
    }
}

fn handle_adversary_action_completion(
    mut adversary_action_completed_events: EventReader<AdversaryActionCompletedEvent>,
    mut phase_changed_events: EventWriter<PhaseChangedEvent>,
    mut cutscene_events: EventWriter<CutsceneStartEvent>,
) {
    for event in adversary_action_completed_events.read() {
        info!("Adversary action completed: {}", event.card_played.title);

        let should_trigger_cutscene = false;

        if should_trigger_cutscene {
            cutscene_events.write(CutsceneStartEvent {
                cutscene_id: format!("adversary_{}", event.card_played.id),
                card_id: Some(event.card_played.id),
                trigger_reason: CutsceneTrigger::AdversaryAction,
            });
        } else {
            // No cutscene, proceed directly to turn over
            phase_changed_events.write(PhaseChangedEvent {
                new_phase: GamePhase::TurnOver,
            });
        }
    }
}

// Modified existing systems to handle new phase flow
fn handle_card_draw(
    mut card_drawn_events: EventReader<CardDrawnEvent>,
    mut phase_state: ResMut<GamePhaseState>,
) {
    for event in card_drawn_events.read() {
        if phase_state.cutscene_active {
            continue;
        }

        phase_state.cards_drawn_count = event.card_count;
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
    mut status_effect_events: EventWriter<StatusEffectAppliedEvent>,
    mut game_state: ResMut<GameState>,
) {
    for event in card_selection_success_events.read() {
        if phase_state.cutscene_active {
            continue;
        }

        phase_state.selected_card_id = Some(event.0.id);

        // Apply card effects first
        let status_events = game_state.apply_card_effects(&event.0);
        for status_event in status_events {
            status_effect_events.write(status_event);
        }

        // Check if cutscene for this card has already been shown
        let cutscene_already_shown = game_state.shown_cutscenes.contains(&event.0.id);

        // Check if cutscene should be triggered based on card type
        let should_trigger_cutscene = !cutscene_already_shown
            && match event.0.card_type {
                CardType::Crisis | CardType::ComboCard => true, // Always show for these types
                _ => rand::random::<f32>() < 0.6, // 60% probability for other card types
            };

        if should_trigger_cutscene {
            // Mark this cutscene as shown
            game_state.shown_cutscenes.insert(event.0.id);

            // Store cutscene to be triggered after action instead of triggering now
            phase_state.pending_post_action_cutscene = Some(PendingCutscene {
                cutscene_id: format!("{}", event.0.id),
                card_id: Some(event.0.id),
                trigger_reason: CutsceneTrigger::CardEffect,
            });
        }

        // Always proceed to character action phase
        phase_changed_events.write(PhaseChangedEvent {
            new_phase: GamePhase::CharacterAction,
        });
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
        phase_state.current_phase = event.new_phase;
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
        if let Some(card_id) = event.card_id {
            if let Some(rpy) = rpy_assets.get(scenario[card_id as usize].id()) {
                novel_events.write(EventStartScenario { ast: rpy.0.clone() });
            }
        }
    }
}

fn handle_cutscene_end(
    mut cutscene_end_events: EventReader<CutsceneEndEvent>,
    mut phase_state: ResMut<GamePhaseState>,
    mut action_completed_events: EventWriter<ActionCompletedEvent>,
    mut turn_over_events: EventWriter<TurnOverEvent>,
    mut phase_changed_events: EventWriter<PhaseChangedEvent>,
) {
    for _event in cutscene_end_events.read() {
        if !phase_state.cutscene_active {
            continue;
        }

        info!("Cutscene ended, resuming game phase");

        phase_state.cutscene_active = false;
        phase_state.pending_cutscene = None;

        match phase_state.current_phase {
            GamePhase::CharacterAction => {
                // If we just finished a post-action cutscene, proceed to adversary card draw
                phase_changed_events.write(PhaseChangedEvent {
                    new_phase: GamePhase::AdversaryCardDraw,
                });
            }
            GamePhase::AdversaryCardSelection => {
                // If we just finished an adversary cutscene, proceed to turn over
                phase_changed_events.write(PhaseChangedEvent {
                    new_phase: GamePhase::TurnOver,
                });
            }
            GamePhase::CardSelection => {
                phase_changed_events.write(PhaseChangedEvent {
                    new_phase: GamePhase::CharacterAction,
                });
            }
            _ => {}
        }

        phase_state.previous_phase = None;
    }
}

fn handle_character_action_phase(
    mut commands: Commands,
    mut phase_changed_events: EventReader<PhaseChangedEvent>,
    mut _navigation_events: EventWriter<NavigateToObjectEvent>,
    phase_state: Res<GamePhaseState>,
    activity_cards_handle: Option<Res<ActivityCardsHandle>>,
    activity_cards_assets: Res<Assets<ActivityCards>>,
) {
    let Some(activity_cards_handle) = activity_cards_handle else {
        warn!("ActivityCardsHandle resource not found");
        return;
    };

    for event in phase_changed_events.read() {
        if event.new_phase == GamePhase::CharacterAction && phase_state.selected_card_id.is_some() {
            if let Some(activity_cards) = activity_cards_assets.get(activity_cards_handle.id()) {
                let selected_card = activity_cards
                    .iter()
                    .find(|card| card.id == phase_state.selected_card_id.unwrap())
                    .unwrap();

                if let Some(required_objects) = selected_card.conditions.required_objects.clone() {
                    for object in required_objects {
                        commands.spawn_task(move || async move {
                            AsyncWorld.sleep(3.0).await;
                            AsyncWorld.send_event(NavigateToObjectEvent {
                                object_name: object,
                            })?;

                            Ok(())
                        });
                        break;
                    }
                } else {
                    commands.spawn_task(move || async move {
                        AsyncWorld.sleep(3.0).await;
                        AsyncWorld.send_event(GoToRandomTile {})?;

                        Ok(())
                    });
                }
            }
        }
    }
}

fn handle_action_completion(
    mut action_completed_events: EventReader<ActionCompletedEvent>,
    mut phase_state: ResMut<GamePhaseState>,
    mut phase_changed_events: EventWriter<PhaseChangedEvent>,
    mut game_step_events: EventWriter<GameStepEvent>,
    mut cutscene_events: EventWriter<CutsceneStartEvent>,
) {
    for event in action_completed_events.read() {
        if phase_state.cutscene_active {
            continue;
        }

        info!("Action completed");

        if let Some(card_played) = event.card_played.to_activity_card() {
            // Apply time cost from played card
            game_step_events.write(GameStepEvent {
                time_delta: card_played.costs.time_cost * 3600.0, // Convert hours to seconds
                sleep_change: 0.0,
                health_change: 0.0,
                mental_health_change: 0.0,
                food_change: 0.0,
            });

            // Check if there's a pending cutscene to trigger after the action
            if let Some(pending_cutscene) = phase_state.pending_post_action_cutscene.take() {
                info!(
                    "Triggering cutscene after action: {}",
                    pending_cutscene.cutscene_id
                );
                cutscene_events.write(CutsceneStartEvent {
                    cutscene_id: pending_cutscene.cutscene_id,
                    card_id: pending_cutscene.card_id,
                    trigger_reason: pending_cutscene.trigger_reason,
                });
            } else {
                // No cutscene pending, proceed to adversary phase
                phase_changed_events.write(PhaseChangedEvent {
                    new_phase: GamePhase::AdversaryCardDraw,
                });
            }
        } else {
            println!("ZZZ");
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

fn handle_phase_changed_turn_over(
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
                    card_id: None,
                });
            }
        }
    }
}

fn handle_turn_over(
    mut commands: Commands,
    mut turn_over_events: EventReader<TurnOverEvent>,
    mut phase_state: ResMut<GamePhaseState>,
    mut game_step_events: EventWriter<GameStepEvent>,
    mut q_cards: ParamSet<(Query<(Entity, &Card<GameCard>)>,)>,
    mut ew_phase_change: EventWriter<PhaseChangedEvent>,
) {
    for _ in turn_over_events.read() {
        if phase_state.cutscene_active {
            continue;
        }

        // Discard cards from table
        for (entity, _) in q_cards.p0().iter() {
            commands.entity(entity).despawn();
        }

        // Clear phase state from previous turn
        phase_state.selected_card_id = None;
        phase_state.cards_drawn_count = 0;
        phase_state.adversary_cards_drawn.clear();
        phase_state.selected_adversary_card = None;

        // Increment turn number
        phase_state.turn_number += 1;

        // Apply passive effects (time passage, resource decay, etc.)
        apply_turn_end_effects(&mut game_step_events);

        ew_phase_change.write(PhaseChangedEvent {
            new_phase: GamePhase::CardDraw,
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
