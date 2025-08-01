use std::time::Duration;

use bevy::color::palettes::css::*;
use bevy::prelude::*;
use bevy_defer::AsyncCommandsExtension;
use bevy_defer::AsyncWorld;
use bevy_la_mesa::events::CardHoverable;
use bevy_la_mesa::events::{CardPress, DeckShuffle, DrawToHand, PlaceCardOnTable, RenderDeck};
use bevy_la_mesa::CardMetadata;
use bevy_la_mesa::{Card, CardOnTable, Hand, PlayArea};
use bevy_la_mesa::{DeckArea, HandArea};
use bevy_tweening::lens::TransformPositionLens;
use bevy_tweening::Animator;
use bevy_tweening::Tween;
use serde::{Deserialize, Deserializer, Serialize};

use crate::logic::CardSelectionSuccess;
use crate::logic::CutsceneEndEvent;
use crate::logic::CutsceneStartEvent;
use crate::logic::GameState;
use crate::logic::PhaseChangedEvent;
use crate::logic::{CardDrawnEvent, CardSelectedEvent, GamePhase, GamePhaseState};

/// Plugin that handles all card-related functionality
pub struct CardSystemPlugin;

#[derive(Event)]
pub struct DragCardsInHandDown {
    pub player: usize,
}

impl Plugin for CardSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_event::<DragCardsInHandDown>()
            .add_systems(
                Update,
                (
                    init_deck,
                    handle_card_draw_phase.after(init_deck),
                    handle_card_selection_attempt,
                    handle_card_selection_success,
                    handle_cutscene_start,
                    handle_cutscene_end,
                    handle_drag_cards_in_hand_down,
                ),
            );
    }
}

pub fn handle_drag_cards_in_hand_down(
    mut commands: Commands,
    mut er_drag_cards: EventReader<DragCardsInHandDown>,
    mut q_hand_cards: Query<(Entity, &mut Transform, &Card<ActivityCard>, &Hand)>,
) {
    for event in er_drag_cards.read() {
        // Find all cards in the specified player's hand
        for (entity, transform, _card, hand) in q_hand_cards.iter_mut() {
            if hand.player == event.player {
                let tween = Tween::new(
                    EaseFunction::CubicIn,
                    Duration::from_millis(300),
                    TransformPositionLens {
                        start: transform.translation,
                        end: transform.translation - Vec3::new(0.0, 0.0, -5.0),
                    },
                );

                commands
                    .entity(entity)
                    .insert((Animator::new(tween), CardHoverable(false)));
            }
        }
    }
}

/// Set up lights, camera, deck area, and hand area for card visualization
fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Ambient light
    commands.insert_resource(AmbientLight {
        color: WHITE.into(),
        brightness: 1000.0,
        ..default()
    });

    // Deck area
    commands.spawn((
        Name::new("Deck 1 -- Play Cards"),
        Transform::from_translation(Vec3::new(14.0, 0.0, 8.0))
            .with_rotation(Quat::from_rotation_y(std::f32::consts::PI / 2.0)),
        DeckArea { marker: 1 },
        Mesh3d(meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Visibility::Hidden,
    ));

    // Hand Area
    commands.spawn((
        Name::new("HandArea - Player 1"),
        Transform::from_translation(Vec3::new(-5.3, 15.7, 3.4)),
        HandArea { player: 1 },
        Mesh3d(meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Visibility::Hidden,
    ));

    // Play Area -- Where card comes to
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_translation(Vec3::new(5.1, 16.0, -0.55)),
        PlayArea {
            marker: 1,
            player: 1,
        },
        Name::new("Play Area 1".to_string()),
        Visibility::Hidden,
    ));
}

/// Initialize the game by loading and rendering the activity cards deck
fn init_deck(
    mut ew_render_deck: EventWriter<RenderDeck<ActivityCard>>,
    q_decks: Query<(Entity, &DeckArea)>,
    activity_cards_handle: Option<Res<ActivityCardsHandle>>,
    activity_cards_assets: Res<Assets<ActivityCards>>,
    phase_state: Res<GamePhaseState>,
    game_state: Res<GameState>,
) {
    if phase_state.current_phase != GamePhase::CardDraw {
        return;
    }

    let Some(activity_cards_handle) = activity_cards_handle else {
        warn!("ActivityCardsHandle resource not found");
        return;
    };

    if let Some(activity_cards) = activity_cards_assets.get(activity_cards_handle.id()) {
        let available_cards = game_state.filter_cards(activity_cards);

        if let Some((deck_entity, _)) = q_decks.iter().next() {
            ew_render_deck.write(RenderDeck::<ActivityCard> {
                deck_entity,
                deck: available_cards,
            });
        }
    }
}

/// Handle the card draw phase - shuffle deck and draw cards
fn handle_card_draw_phase(
    mut commands: Commands,
    phase_state: Res<GamePhaseState>,
    q_decks: Query<(Entity, &DeckArea)>,
    q_cards_on_table: Query<(Entity, &Card<ActivityCard>, &CardOnTable)>,
    mut last_turn: Local<u32>,
    mut phase_changed_events: EventWriter<PhaseChangedEvent>,
) {
    // Only trigger when we enter the CardDraw phase
    if phase_state.current_phase != GamePhase::CardDraw {
        return;
    }

    // Prevent triggering multiple times for the same turn
    if *last_turn == phase_state.turn_number {
        return;
    }

    let Some((deck_entity, _)) = q_decks.iter().next() else {
        warn!("No deck found for card draw");
        return;
    };

    let n_cards_on_table = q_cards_on_table.iter().len();
    let cards_to_draw = 5 - n_cards_on_table;

    if cards_to_draw <= 0 {
        warn!("No cards to draw, table is full");
        return;
    }

    *last_turn = phase_state.turn_number;

    // Draw cards after a delay
    commands.spawn_task(move || async move {
        AsyncWorld.sleep(1.0).await;

        // Shuffle the deck first
        AsyncWorld.send_event(DeckShuffle {
            deck_entity,
            duration: 50,
        })?;

        AsyncWorld.sleep(2.0).await;

        // Send draw event
        AsyncWorld.send_event(DrawToHand {
            deck_entity,
            num_cards: cards_to_draw,
            player: 1,
        })?;

        // Send phase event
        AsyncWorld.send_event(CardDrawnEvent {
            card_count: cards_to_draw,
        })?;

        Ok(())
    });

    phase_changed_events.write(PhaseChangedEvent {
        new_phase: GamePhase::CardSelection,
    });
}

/// Handle placing card on table after selection
pub fn handle_card_selection_attempt(
    mut card_press: EventReader<CardPress>,
    mut ew_card_selected: EventWriter<CardSelectedEvent>,
    phase_state: Res<GamePhaseState>,
    mut q_cards: ParamSet<(
        Query<(Entity, &Card<ActivityCard>, &CardOnTable)>,
        Query<(Entity, &Card<ActivityCard>, &Hand)>,
    )>,
) {
    // Only place cards during CharacterAction phase
    if phase_state.current_phase != GamePhase::CardSelection {
        return;
    }

    for event in card_press.read() {
        let p0 = q_cards.p0();
        let n_cards_on_table = p0.iter().len();

        // Skip if card is already on table
        if p0.get(event.entity).is_ok() {
            continue;
        }

        let p1 = q_cards.p1();
        if let Ok((_, card, _)) = p1.get(event.entity)
            && n_cards_on_table < 1
        {
            ew_card_selected.write(CardSelectedEvent(card.data.clone()));
        }
    }
}

pub fn handle_card_selection_success(
    commands: Commands,
    mut ew_place_card_on_table: EventWriter<PlaceCardOnTable>,
    mut er_card_selction_success: EventReader<CardSelectionSuccess>,
    mut ew_drag_cards: EventWriter<DragCardsInHandDown>,
    mut q_cards: ParamSet<(
        Query<(Entity, &Card<ActivityCard>, &CardOnTable)>,
        Query<(Entity, &Card<ActivityCard>, &Hand)>,
    )>,
) {
    for event in er_card_selction_success.read() {
        let selected_card_entity = q_cards
            .p1()
            .iter()
            .find(|(_, card, _)| card.data.id == event.0.id)
            .unwrap()
            .0;

        let n_cards_on_table = q_cards.p0().iter().len();

        ew_place_card_on_table.write(PlaceCardOnTable {
            card_entity: selected_card_entity,
            player: 1,
            marker: n_cards_on_table + 1,
        });

        ew_drag_cards.write(DragCardsInHandDown { player: 1 });
    }
}

fn handle_cutscene_start(
    mut commands: Commands,
    mut er_cutscene_start: EventReader<CutsceneStartEvent>,
    q_cards: Query<(Entity, &Card<ActivityCard>)>,
) {
    for _ in er_cutscene_start.read() {
        for (entity, _) in q_cards.iter() {
            commands.entity(entity).insert(Visibility::Hidden);
        }
    }
}

fn handle_cutscene_end(
    mut commands: Commands,
    mut er_cutscene_start: EventReader<CutsceneEndEvent>,
    q_cards: Query<(Entity, &Card<ActivityCard>)>,
) {
    for _ in er_cutscene_start.read() {
        for (entity, _) in q_cards.iter() {
            commands.entity(entity).insert(Visibility::Inherited);
        }
    }
}

// Card-related types and definitions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActivityCard {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub category: String,
    pub conditions: CardConditions,
    pub costs: CardCosts,
    pub effects: ResourceEffects,
    pub status_effects: Vec<StatusEffectApplication>,
    pub card_type: CardType,
    pub availability: CardAvailability,
    pub flavor_text: String,
    pub one_time_use: bool,
    pub cooldown: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CardConditions {
    pub min_sleep: Option<f32>,
    pub min_health: Option<f32>,
    pub min_mental: Option<f32>,
    pub min_food: Option<f32>,
    pub max_sleep: Option<f32>,
    pub max_health: Option<f32>,
    pub max_mental: Option<f32>,
    pub max_food: Option<f32>,
    pub required_mood: Option<Mood>,
    pub forbidden_mood: Option<Mood>,
    pub time_of_day: Option<Vec<TimeOfDay>>,
    pub day_range: Option<(u32, u32)>,
    pub required_objects: Option<Vec<String>>,
    pub crisis_level: Option<CrisisLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CardCosts {
    pub sleep_cost: f32,
    pub health_cost: f32,
    pub mental_cost: f32,
    pub food_cost: f32,
    pub time_cost: f32,
    pub additional_costs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceEffects {
    pub sleep: f32,
    pub health: f32,
    pub mental: f32,
    pub food: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffectApplication {
    pub effect: StatusEffect,
    pub duration: u32,
    pub intensity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CardType {
    #[default]
    BasicNeed,
    Entertainment,
    Social,
    Crisis,
    TimeSpecific,
    Delusion,
    LLMGenerated,
    ThoughtCard,
    MemoryCard,
    ImpulseCard,
    ComboCard,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CardAvailability {
    #[default]
    Always,
    ConditionalOnly,
    CrisisOnly,
    LLMOnly,
    OneTime,
    DailyReset,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum StatusEffect {
    Insomnia(u32),
    Sick(u32),
    Motivated(u32),
    Overwhelmed(u32),
    Addicted(String, u32),
    Exhausted(u32),
    Anxious(u32),
    Depressed(u32),
    Manic(u32),
    Stable(u32),
    Focused(u32),
    Hungry(u32),
}

impl<'de> Deserialize<'de> for StatusEffect {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        // Parse the string and create the appropriate enum variant with default duration
        match s.as_str() {
            "Insomnia" => Ok(StatusEffect::Insomnia(0)),
            "Sick" => Ok(StatusEffect::Sick(0)),
            "Motivated" => Ok(StatusEffect::Motivated(0)),
            "Overwhelmed" => Ok(StatusEffect::Overwhelmed(0)),
            "Exhausted" => Ok(StatusEffect::Exhausted(0)),
            "Anxious" => Ok(StatusEffect::Anxious(0)),
            "Depressed" => Ok(StatusEffect::Depressed(0)),
            "Manic" => Ok(StatusEffect::Manic(0)),
            "Stable" => Ok(StatusEffect::Stable(0)),
            "Focused" => Ok(StatusEffect::Focused(0)),
            "Hungry" => Ok(StatusEffect::Hungry(0)),
            s if s.starts_with("Addicted") => {
                // Handle "Addicted" or "Addicted(substance)" format
                Ok(StatusEffect::Addicted("generic".to_string(), 0))
            }
            _ => Err(serde::de::Error::custom(format!(
                "Unknown status effect: {s}"
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Mood {
    Depressed,
    Anxious,
    Tired,
    Neutral,
    Content,
    Manic,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TimeOfDay {
    EarlyMorning, // 5-9
    Morning,      // 9-12
    Afternoon,    // 12-17
    Evening,      // 17-20
    Night,        // 20-24
    LateNight,    // 0-5
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum CrisisLevel {
    None,
    Mild,
    Moderate,
    Severe,
    Critical,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ResourceType {
    Sleep,
    Health,
    Mental,
    Food,
}

#[derive(Deserialize, Asset, TypePath, Deref, DerefMut)]
pub struct ActivityCards(pub Vec<ActivityCard>);

/// Resource for storing the handle to the activity cards asset
#[derive(Resource, Deref, DerefMut)]
pub struct ActivityCardsHandle(pub Handle<ActivityCards>);

impl CardMetadata for ActivityCard {
    type Output = ActivityCard;

    fn front_image_filename(&self) -> String {
        format!("cards/card-{}.png", self.id)
    }

    fn back_image_filename(&self) -> String {
        "cards/Back_1.png".into()
    }
}
