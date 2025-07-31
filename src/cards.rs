use std::time::Duration;

use avian2d::math::PI;
use bevy::color::palettes::css::*;
use bevy::prelude::*;
use bevy_defer::AsyncCommandsExtension;
use bevy_defer::AsyncWorld;
use bevy_la_mesa::CardMetadata;
use bevy_la_mesa::events::CardHoverable;
use bevy_la_mesa::events::{CardPress, DeckShuffle, DrawToHand, PlaceCardOnTable, RenderDeck};
use bevy_la_mesa::{Card, CardOnTable, Hand, PlayArea};
use bevy_la_mesa::{DeckArea, HandArea};
use bevy_tweening::Animator;
use bevy_tweening::Tween;
use bevy_tweening::lens::TransformPositionLens;
use serde::{Deserialize, Deserializer, Serialize};

use crate::AppState;
use crate::logic::AdversaryCardDrawnEvent;
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

#[derive(Event)]
pub struct DragCardsInHandUp {
    pub player: usize,
}

impl Plugin for CardSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup)
            .add_event::<DragCardsInHandDown>()
            .add_event::<DragCardsInHandUp>()
            .add_systems(
                Update,
                (
                    init_player_deck,
                    handle_player_card_draw_phase.after(init_player_deck),
                    init_adversary_deck,
                    handle_adversary_card_draw_phase.after(init_adversary_deck),
                    handle_card_selection_attempt,
                    handle_card_selection_success,
                    handle_cutscene_start,
                    handle_cutscene_end,
                    handle_drag_cards_in_hand_down,
                    handle_drag_cards_in_hand_up,
                )
                    .run_if(in_state(AppState::Game)),
            );
    }
}

pub fn handle_drag_cards_in_hand_down(
    mut commands: Commands,
    mut er_drag_cards: EventReader<DragCardsInHandDown>,
    mut q_hand_cards: Query<(Entity, &mut Transform, &Card<GameCard>, &Hand)>,
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
                    .insert(Animator::new(tween))
                    .insert(CardHoverable(false));
            }
        }
    }
}

pub fn handle_drag_cards_in_hand_up(
    mut commands: Commands,
    mut er_drag_cards: EventReader<DragCardsInHandUp>,
    mut q_hand_cards: Query<(Entity, &mut Transform, &Card<GameCard>, &Hand)>,
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
                        end: transform.translation - Vec3::new(0.0, 0.0, 5.0),
                    },
                );

                commands
                    .entity(entity)
                    .insert(Animator::new(tween))
                    .insert(CardHoverable(false));
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
        Transform::from_translation(Vec3::new(7.8, 0.0, 8.0))
            .with_rotation(Quat::from_rotation_y(std::f32::consts::PI / 2.0)),
        DeckArea { marker: 1 },
        Mesh3d(meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Visibility::Hidden,
    ));

    commands.spawn((
        Name::new("Deck 2 -- Play Cards"),
        Transform::from_translation(Vec3::new(3.3, 0.0, 8.0))
            .with_rotation(Quat::from_rotation_y(std::f32::consts::PI / 2.0)),
        DeckArea { marker: 2 },
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

    commands.spawn((
        Name::new("HandArea - Player 2"),
        Transform::from_translation(Vec3::new(-5.3, 15.7, -4.0))
            .with_rotation(Quat::from_rotation_y(PI)),
        HandArea { player: 2 },
        Mesh3d(meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Visibility::Hidden,
    ));

    // Play Area -- Where card comes to
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_translation(Vec3::new(5.1, 16.0, -1.8)),
        PlayArea {
            marker: 1,
            player: 1,
        },
        Name::new("Play Area 1".to_string()),
        Visibility::Hidden,
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_translation(Vec3::new(5.1, 16.0, 1.8)),
        PlayArea {
            marker: 1,
            player: 2,
        },
        Name::new("Play Area 2".to_string()),
        Visibility::Hidden,
    ));
}

fn init_player_deck(
    mut ew_render_deck: EventWriter<RenderDeck<GameCard>>,
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
        warn!("GameCardsHandle resource not found");
        return;
    };

    if let Some(activity_cards) = activity_cards_assets.get(activity_cards_handle.id()) {
        // Filter to only activity cards
        let game_cards = GameCard::from_activity_cards(activity_cards.0.clone());
        let available_cards = game_state.filter_cards(&game_cards);

        if let Some((deck_entity, _)) = q_decks.iter().find(|(_, deck)| deck.marker == 1) {
            ew_render_deck.write(RenderDeck::<GameCard> {
                deck_entity,
                deck: available_cards,
            });
        }
    }
}

fn handle_player_card_draw_phase(
    mut commands: Commands,
    phase_state: Res<GamePhaseState>,
    q_decks: Query<(Entity, &DeckArea)>,
    q_cards_on_table: Query<(Entity, &Card<GameCard>, &CardOnTable)>,
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
    if let Some((deck_entity, _)) = q_decks.iter().find(|(_, deck)| deck.marker == 1) {
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
}

fn init_adversary_deck(
    mut ew_render_deck: EventWriter<RenderDeck<GameCard>>,
    q_decks: Query<(Entity, &DeckArea)>,
    schizophrenic_cards_handle: Option<Res<SchizophrenicCardsHandle>>,
    schizophrenic_cards_assets: Res<Assets<SchizophrenicCards>>,
    phase_state: Res<GamePhaseState>,
) {
    if phase_state.current_phase != GamePhase::AdversaryCardDraw {
        return;
    }

    let Some(schizophrenic_cards_handle) = schizophrenic_cards_handle else {
        warn!("GameCardsHandle resource not found");
        return;
    };

    if let Some(schizophrenic_cards) =
        schizophrenic_cards_assets.get(schizophrenic_cards_handle.id())
    {
        let game_cards = GameCard::from_schizophrenic_cards(schizophrenic_cards.0.clone());
        if let Some((deck_entity, _)) = q_decks.iter().find(|(_, deck)| deck.marker == 2) {
            ew_render_deck.write(RenderDeck::<GameCard> {
                deck_entity,
                deck: game_cards,
            });
        }
    }
}

fn handle_adversary_card_draw_phase(
    mut commands: Commands,
    phase_state: Res<GamePhaseState>,
    q_decks: Query<(Entity, &DeckArea)>,
    q_cards_on_table: Query<(Entity, &Card<GameCard>, &CardOnTable)>,
    mut last_turn: Local<u32>,
    mut phase_changed_events: EventWriter<PhaseChangedEvent>,
) {
    // Only trigger when we enter the AdversaryCardDraw phase
    if phase_state.current_phase != GamePhase::AdversaryCardDraw {
        return;
    }

    // Prevent triggering multiple times for the same turn
    if *last_turn == phase_state.turn_number {
        return;
    }

    // Find the adversary deck (marker 2)
    if let Some((deck_entity, _)) = q_decks.iter().find(|(_, deck)| deck.marker == 2) {
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

            // Send draw event for adversary (player 2)
            AsyncWorld.send_event(DrawToHand {
                deck_entity,
                num_cards: cards_to_draw + 1,
                player: 2,
            })?;

            // Send phase event
            AsyncWorld.send_event(AdversaryCardDrawnEvent {
                card_count: cards_to_draw,
            })?;

            Ok(())
        });

        phase_changed_events.write(PhaseChangedEvent {
            new_phase: GamePhase::AdversaryCardSelection,
        });
    } else {
        warn!("No adversary deck found for card draw");
    }
}

/// Handle placing card on table after selection
pub fn handle_card_selection_attempt(
    mut card_press: EventReader<CardPress>,
    mut ew_card_selected: EventWriter<CardSelectedEvent>,
    phase_state: Res<GamePhaseState>,
    mut q_cards: ParamSet<(
        Query<(Entity, &Card<GameCard>, &CardOnTable)>,
        Query<(Entity, &Card<GameCard>, &Hand)>,
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
            println!("selected activity card {:?}", card.data);

            if let CardVariant::Activity(activity_card) = &card.data.card_variant {
                ew_card_selected.write(CardSelectedEvent(activity_card.clone()));
            }
        }
    }
}

pub fn handle_card_selection_success(
    commands: Commands,
    mut ew_place_card_on_table: EventWriter<PlaceCardOnTable>,
    mut er_card_selction_success: EventReader<CardSelectionSuccess>,
    mut ew_drag_cards: EventWriter<DragCardsInHandDown>,
    mut q_cards: ParamSet<(
        Query<(Entity, &Card<GameCard>, &CardOnTable)>,
        Query<(Entity, &Card<GameCard>, &Hand)>,
    )>,
) {
    for event in er_card_selction_success.read() {
        let selected_card_entity = q_cards
            .p1()
            .iter()
            .find(|(_, card, _)| {
                if let CardVariant::Activity(activity_card) = &card.data.card_variant {
                    activity_card.id == event.0.id
                } else {
                    false
                }
            })
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
    q_cards: Query<(Entity, &Card<GameCard>)>,
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
    q_cards: Query<(Entity, &Card<GameCard>)>,
) {
    for _ in er_cutscene_start.read() {
        for (entity, _) in q_cards.iter() {
            commands.entity(entity).insert(Visibility::Inherited);
        }
    }
}

// Unified Card System

/// Unified card structure that can represent both activity and schizophrenic cards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameCard {
    pub card_variant: CardVariant,
}

impl Default for GameCard {
    fn default() -> Self {
        Self {
            card_variant: CardVariant::Activity(ActivityCard::default()),
        }
    }
}

/// Enum to distinguish between different card types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CardVariant {
    Activity(ActivityCard),
    Schizophrenic(SchizophrenicCard),
}

// Activity Card Types
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

// Schizophrenic Card Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchizophrenicCard {
    pub id: u32,
    pub card_name: String,
    pub scenario_id: String,
    pub title: String,
    pub conditions: SpectrumConditions,
    pub setup: String,
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

// Conditions specific to spectrum cards
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SpectrumConditions {
    pub trigger_symptoms: Vec<String>,
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
    Spectrum, // New type for schizophrenic cards
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
    EarlyMorning,
    Morning,
    Afternoon,
    Evening,
    Night,
    LateNight,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Deserialize, Asset, TypePath, Deref, DerefMut)]
pub struct SchizophrenicCards(pub Vec<SchizophrenicCard>);

#[derive(Asset, TypePath, Deref, DerefMut)]
pub struct GameCards(pub Vec<ActivityCard>);

#[derive(Resource, Deref, DerefMut)]
pub struct ActivityCardsHandle(pub Handle<ActivityCards>);

#[derive(Resource, Deref, DerefMut)]
pub struct SchizophrenicCardsHandle(pub Handle<SchizophrenicCards>);

impl CardMetadata for GameCard {
    type Output = GameCard;

    fn front_image_filename(&self) -> String {
        match &self.card_variant {
            CardVariant::Activity(_) => format!("cards/card-{}.png", self.id()),
            CardVariant::Schizophrenic(_) => format!("schizo_cards/card-{}.png", self.id()),
        }
    }

    fn back_image_filename(&self) -> String {
        match &self.card_variant {
            CardVariant::Activity(_) => "cards/Back_1.png".into(),
            CardVariant::Schizophrenic(_) => "cards/Back_5.png".into(),
        }
    }
}

impl GameCard {
    pub fn id(&self) -> u32 {
        match self.card_variant.clone() {
            CardVariant::Activity(activity_card) => activity_card.clone().id,
            CardVariant::Schizophrenic(schizophrenic_card) => schizophrenic_card.clone().id,
        }
    }

    /// Convert a list of ActivityCards to GameCards
    pub fn from_activity_cards(activity_cards: Vec<ActivityCard>) -> Vec<GameCard> {
        activity_cards
            .into_iter()
            .map(|activity_card| GameCard {
                card_variant: CardVariant::Activity(activity_card),
            })
            .collect()
    }

    /// Convert a list of SchizophrenicCards to GameCards
    pub fn from_schizophrenic_cards(schizo_cards: Vec<SchizophrenicCard>) -> Vec<GameCard> {
        schizo_cards
            .into_iter()
            .map(|schizo_card| GameCard {
                card_variant: CardVariant::Schizophrenic(schizo_card),
            })
            .collect()
    }

    /// Convert both lists and combine them into a single GameCards collection
    pub fn from_both_card_types(
        activity_cards: Vec<ActivityCard>,
        schizo_cards: Vec<SchizophrenicCard>,
    ) -> Vec<GameCard> {
        let mut game_cards = Self::from_activity_cards(activity_cards);
        game_cards.extend(Self::from_schizophrenic_cards(schizo_cards));
        game_cards
    }

    pub fn to_activity_card(&self) -> Option<&ActivityCard> {
        match &self.card_variant {
            CardVariant::Activity(activity_card) => Some(activity_card),
            CardVariant::Schizophrenic(_) => None,
        }
    }

    /// Extract SchizophrenicCard if this is a schizophrenic card
    pub fn to_schizophrenic_card(&self) -> Option<&SchizophrenicCard> {
        match &self.card_variant {
            CardVariant::Activity(_) => None,
            CardVariant::Schizophrenic(schizo_card) => Some(schizo_card),
        }
    }

    /// Extract ActivityCard by consuming the GameCard
    pub fn into_activity_card(self) -> Option<ActivityCard> {
        match self.card_variant {
            CardVariant::Activity(activity_card) => Some(activity_card),
            CardVariant::Schizophrenic(_) => None,
        }
    }

    /// Extract SchizophrenicCard by consuming the GameCard
    pub fn into_schizophrenic_card(self) -> Option<SchizophrenicCard> {
        match self.card_variant {
            CardVariant::Activity(_) => None,
            CardVariant::Schizophrenic(schizo_card) => Some(schizo_card),
        }
    }
}
