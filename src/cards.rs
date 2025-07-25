use std::collections::HashMap;

use bevy::asset::Asset;
use bevy::color::palettes::css::*;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy_defer::AsyncCommandsExtension;
use bevy_defer::AsyncWorld;
use bevy_la_mesa::events::{
    AlignCardsInHand, CardPress, DeckShuffle, DiscardCardToDeck, DrawToHand, PlaceCardOnTable,
    RenderDeck,
};
use bevy_la_mesa::{Card, CardMetadata, CardOnTable, Hand, PlayArea};
use bevy_la_mesa::{DeckArea, HandArea};
use serde::Deserialize;

use crate::logic::CutsceneEndEvent;
use crate::logic::CutsceneStartEvent;
// Import your game logic events
use crate::logic::{CardDrawnEvent, CardSelectedEvent, GamePhase, GamePhaseState};

/// Plugin that handles all card-related functionality
pub struct CardSystemPlugin;

impl Plugin for CardSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                init_cards,
                handle_card_draw_phase,
                handle_place_card_on_table,
                handle_cutscene_start,
                handle_cutscene_end,
            ),
        );
    }
}

/// Resource for storing the handle to the activity cards asset
#[derive(Resource, Deref, DerefMut)]
pub struct ActivityCardsHandle(pub Handle<ActivityCards>);

#[derive(Clone, Debug, Default, Deserialize)]
pub struct ActivityCard {
    pub card_number: usize,
    pub name: String,
    pub description: String,
    pub category: String,
    pub resource_effects: HashMap<String, f32>,
    pub time_cost: f32,
    pub card_type: String,
    pub availability: String,
}

#[derive(Deserialize, Asset, TypePath, Deref, DerefMut)]
pub struct ActivityCards(pub Vec<ActivityCard>);

impl CardMetadata for ActivityCard {
    type Output = ActivityCard;

    fn front_image_filename(&self) -> String {
        format!("cards/card-{}.png", self.card_number - 1)
    }

    fn back_image_filename(&self) -> String {
        "cards/Back_1.png".into()
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
        Transform::from_translation(Vec3::new(14.0, 0.0, 7.5))
            .with_rotation(Quat::from_rotation_y(std::f32::consts::PI / 2.0)),
        DeckArea { marker: 1 },
        Mesh3d(meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Visibility::Hidden,
    ));

    // Hand Area
    commands.spawn((
        Name::new("HandArea - Player 1"),
        Transform::from_translation(Vec3::new(-5.0, 8.5, 5.0)),
        HandArea { player: 1 },
        Mesh3d(meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Visibility::Hidden,
    ));

    // Play Area -- Where card comes to
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_translation(Vec3::new(-5.6, 15.1, 2.15)),
        PlayArea {
            marker: 1,
            player: 1,
        },
        Name::new(format!("Play Area 1")),
        Visibility::Hidden,
    ));
}

/// Initialize the game by loading and rendering the activity cards deck
fn init_cards(
    mut ew_render_deck: EventWriter<RenderDeck<ActivityCard>>,
    q_decks: Query<(Entity, &DeckArea)>,
    activity_cards_handle: Option<Res<ActivityCardsHandle>>,
    activity_cards_assets: Res<Assets<ActivityCards>>,
    mut has_run: Local<bool>,
) {
    if *has_run {
        return;
    }

    let Some(activity_cards_handle) = activity_cards_handle else {
        warn!("ActivityCardsHandle resource not found");
        return;
    };

    if let Some(activity_cards) = activity_cards_assets.get(activity_cards_handle.id()) {
        if let Some((deck_entity, _)) = q_decks.iter().next() {
            ew_render_deck.write(RenderDeck::<ActivityCard> {
                deck_entity,
                deck: activity_cards.to_vec(),
            });

            *has_run = true;
        }
    }
}

/// Handle the card draw phase - shuffle deck and draw cards
fn handle_card_draw_phase(
    mut commands: Commands,
    phase_state: Res<GamePhaseState>,
    q_decks: Query<(Entity, &DeckArea)>,
    q_cards_on_table: Query<(Entity, &Card<ActivityCard>, &CardOnTable)>,
    mut ew_shuffle: EventWriter<DeckShuffle>,
    mut ew_card_drawn: EventWriter<CardDrawnEvent>,
    mut last_turn: Local<u32>,
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

    // Shuffle the deck first
    ew_shuffle.write(DeckShuffle {
        deck_entity,
        duration: 8,
    });

    // Draw cards after a delay
    commands.spawn_task(move || async move {
        AsyncWorld.sleep(0.5).await;

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
}

/// Handle placing card on table after selection
pub fn handle_place_card_on_table(
    mut commands: Commands,
    mut card_press: EventReader<CardPress>,
    mut ew_place_card_on_table: EventWriter<PlaceCardOnTable>,
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
        if let Ok((_, card, _)) = p1.get(event.entity) {
            // Send card selection event with card number as ID
            ew_card_selected.write(CardSelectedEvent(card.data.clone()));
        }

        // Only allow one card on table at a time for now
        if n_cards_on_table < 1 {
            ew_place_card_on_table.write(PlaceCardOnTable {
                card_entity: event.entity,
                player: 1,
                marker: n_cards_on_table + 1,
            });

            // Align remaining cards in hand
            commands.spawn_task(move || async move {
                AsyncWorld.sleep(0.5).await;
                AsyncWorld.send_event(AlignCardsInHand { player: 1 })?;
                Ok(())
            });
        }
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
