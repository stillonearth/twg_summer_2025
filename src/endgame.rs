use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    cards::{CrisisLevel, Mood},
    logic::GameState,
    ui::EndTurnEvent,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndGameScenario {
    pub id: String,
    pub title: String,
    pub trigger_conditions: TriggerConditions,
    pub description: String,
    pub image_prompt: String,
    pub ending_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerConditions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mental_health: Option<ResourceCondition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<ResourceCondition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sleep: Option<ResourceCondition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub food: Option<ResourceCondition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crisis_level: Option<CrisisLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_effects_present: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mood: Option<Mood>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consecutive_stable_days: Option<ResourceCondition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_number: Option<ResourceCondition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_trigger_symptoms: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResourceCondition(pub u32, pub u32);

impl<'de> Deserialize<'de> for ResourceCondition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, Visitor};
        use std::fmt;

        struct ResourceConditionVisitor;

        impl<'de> Visitor<'de> for ResourceConditionVisitor {
            type Value = ResourceCondition;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string condition like '<=0' or '30-50', or a number")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value.starts_with("<=") {
                    let threshold: u32 = value[2..].parse().map_err(de::Error::custom)?;
                    Ok(ResourceCondition(0, threshold))
                } else if value.starts_with(">=") {
                    let threshold: u32 = value[2..].parse().map_err(de::Error::custom)?;
                    Ok(ResourceCondition(threshold, 100))
                } else if value.starts_with("<") {
                    let threshold: u32 = value[1..].parse().map_err(de::Error::custom)?;
                    Ok(ResourceCondition(0, threshold.saturating_sub(1)))
                } else if value.starts_with(">") {
                    let threshold: u32 = value[1..].parse().map_err(de::Error::custom)?;
                    Ok(ResourceCondition(threshold + 1, 100))
                } else if value.contains('-') {
                    // Range condition like "30-50"
                    let parts: Vec<&str> = value.split('-').collect();
                    if parts.len() == 2 {
                        let min: u32 = parts[0].trim().parse().map_err(de::Error::custom)?;
                        let max: u32 = parts[1].trim().parse().map_err(de::Error::custom)?;
                        Ok(ResourceCondition(min, max))
                    } else {
                        Err(de::Error::custom("Invalid range format"))
                    }
                } else {
                    // Try to parse as a simple number
                    let number: u32 = value.parse().map_err(de::Error::custom)?;
                    Ok(ResourceCondition(number, number))
                }
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let number = value as u32;
                Ok(ResourceCondition(number, number))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let number = value as u32;
                Ok(ResourceCondition(number, number))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let number = value as u32;
                Ok(ResourceCondition(number, number))
            }
        }

        deserializer.deserialize_any(ResourceConditionVisitor)
    }
}

impl ResourceCondition {
    pub fn evaluate(&self, value: f32) -> bool {
        let value_u32 = value as u32;
        value_u32 >= self.0 && value_u32 <= self.1
    }
}

impl TriggerConditions {
    pub fn evaluate(&self, game_state: &GameState) -> bool {
        // Check mental health condition
        if let Some(ref condition) = self.mental_health {
            if !condition.evaluate(game_state.mental_health) {
                return false;
            }
        }

        // Check health condition
        if let Some(ref condition) = self.health {
            if !condition.evaluate(game_state.health) {
                return false;
            }
        }

        // Check sleep condition
        if let Some(ref condition) = self.sleep {
            if !condition.evaluate(game_state.sleep) {
                return false;
            }
        }

        // Check food condition
        if let Some(ref condition) = self.food {
            if !condition.evaluate(game_state.food) {
                return false;
            }
        }

        // Check crisis level
        if let Some(ref required_crisis) = self.crisis_level {
            if game_state.crisis_level != *required_crisis {
                return false;
            }
        }

        // Check mood
        if let Some(ref required_mood) = self.mood {
            if game_state.current_mood != *required_mood {
                return false;
            }
        }

        // Check consecutive stable days
        if let Some(ref condition) = self.consecutive_stable_days {
            if !condition.evaluate(game_state.consecutive_stable_days as f32) {
                return false;
            }
        }

        if let Some(ref condition) = self.turn_number {
            if !condition.evaluate(game_state.turn_number() as f32) {
                return false;
            }
        }

        // Check status effects
        if let Some(ref required_effects) = self.status_effects_present {
            for required_effect in required_effects {
                let has_effect = game_state.status_effects.iter().any(|status| {
                    // Assuming status effects have a string representation or enum variant name
                    format!("{:?}", status.effect)
                        .to_lowercase()
                        .contains(&required_effect.to_lowercase())
                });
                if !has_effect {
                    return false;
                }
            }
        }

        // Check active trigger symptoms
        if let Some(ref required_symptoms) = self.active_trigger_symptoms {
            for required_symptom in required_symptoms {
                if !game_state
                    .active_trigger_symptoms
                    .contains(required_symptom)
                {
                    return false;
                }
            }
        }

        true
    }
}

pub fn check_endgame_conditions(
    scenarios: &[EndGameScenario],
    game_state: &GameState,
) -> Option<EndGameScenario> {
    let scenario = scenarios
        .iter()
        .find(|scenario| scenario.trigger_conditions.evaluate(game_state));
    if let Some(sc) = scenario {
        return Some(sc.clone());
    }
    return None;
}

#[derive(Deserialize, Asset, TypePath, Deref, DerefMut)]
pub struct EndGameScenarios(pub Vec<EndGameScenario>);

#[derive(Resource, Deref, DerefMut)]
pub struct EndGameScenariosHandle(pub Handle<EndGameScenarios>);

pub struct EndGamePlugin;

impl Plugin for EndGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EndGameEvent>()
            .add_systems(Update, check_game_over_on_turn_end);
    }
}

/// Event fired when the game ends
#[derive(Event)]
pub struct EndGameEvent {
    pub scenario_id: String,
}

/// System that checks for game over conditions when turn ends
pub fn check_game_over_on_turn_end(
    mut turn_end_events: EventReader<EndTurnEvent>,
    mut end_game_events: EventWriter<EndGameEvent>,
    mut game_state: ResMut<GameState>,
    end_game_scenarios_handle: Option<Res<EndGameScenariosHandle>>,
    end_game_scenarios_assets: Res<Assets<EndGameScenarios>>,
) {
    let Some(end_game_scenarios_handle) = end_game_scenarios_handle else {
        warn!("EndGameScenariosHandle resource not found");
        return;
    };

    // Only check on turn end events
    for _event in turn_end_events.read() {
        // Get the end game scenarios

        let Some(end_game_scenarios) =
            end_game_scenarios_assets.get(end_game_scenarios_handle.id())
        else {
            warn!("EndGameScenarios asset not loaded");
            return;
        };

        // Check if any end game scenario conditions are met
        if let Some(matched_scenario) = check_endgame_conditions(&end_game_scenarios, &game_state) {
            end_game_events.write(EndGameEvent {
                scenario_id: matched_scenario.id.clone(),
            });

            info!(
                "Game Over! Scenario: {} - {}",
                matched_scenario.id, matched_scenario.title
            );
            info!("Ending: {}", matched_scenario.ending_text);
        }
    }
}
