use std::collections::HashMap;

use bevy::asset::Asset;
use bevy::color::palettes::css::*;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy_la_mesa::events::{DeckShuffle, DrawToHand, RenderDeck};
use bevy_la_mesa::CardMetadata;
use bevy_la_mesa::{DeckArea, HandArea};
use serde::Deserialize;

/// Plugin that handles all card-related functionality
pub struct CardSystemPlugin;

impl Plugin for CardSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_cards_3d, setup_ui))
            .add_systems(Update, (handle_card_buttons, init_cards));
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
        format!("cards/card-{}.png", self.card_number)
    }

    fn back_image_filename(&self) -> String {
        "cards/Back_1.png".into()
    }
}

/// Component marker for the shuffle deck button
#[derive(Component)]
pub struct ButtonShuffleDeck;

/// Component marker for the draw hand button
#[derive(Component)]
pub struct ButtonDrawHand;

/// Button color constants
mod button_colors {
    use bevy::prelude::Color;

    pub const NORMAL: Color = Color::srgb(0.15, 0.15, 0.15);
    pub const HOVERED: Color = Color::srgb(0.25, 0.25, 0.25);
    pub const PRESSED: Color = Color::srgb(0.35, 0.75, 0.35);
}

/// Set up lights, camera, deck area, and hand area for card visualization
fn setup_cards_3d(
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

    // 3D camera for card rendering
    commands.spawn((
        Name::new("Card Camera"),
        Camera3d::default(),
        Camera {
            order: 2,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        Transform::from_xyz(0.0, 15.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Deck area
    commands.spawn((
        Name::new("Deck 1 -- Play Cards"),
        Transform::from_translation(Vec3::new(12.0, 0.0, 0.0))
            .with_rotation(Quat::from_rotation_y(std::f32::consts::PI / 2.0)),
        DeckArea { marker: 1 },
    ));

    // Hand area for player 1
    commands.spawn((
        Name::new("HandArea - Player 1"),
        Transform::from_translation(Vec3::new(0.0, -2.2, 5.8))
            .with_rotation(Quat::from_rotation_x(std::f32::consts::PI / 4.0)),
        HandArea { player: 1 },
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
        println!("n activity cards {}", activity_cards.len());
        if let Some((deck_entity, _)) = q_decks.iter().next() {
            ew_render_deck.write(RenderDeck::<ActivityCard> {
                deck_entity,
                deck: activity_cards.to_vec(),
            });

            *has_run = true;
        }
    }
}

/// Handle button interactions for shuffle and draw actions
fn handle_card_buttons(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor, &mut BorderColor),
        Changed<Interaction>,
    >,
    button_types: Query<(Entity, Option<&ButtonShuffleDeck>, Option<&ButtonDrawHand>)>,
    decks: Query<(Entity, &DeckArea)>,
    mut ew_shuffle: EventWriter<DeckShuffle>,
    mut ew_draw: EventWriter<DrawToHand>,
) {
    let Some((deck_entity, _)) = decks.iter().next() else {
        return;
    };

    for (entity, interaction, mut bg_color, mut border_color) in interaction_query.iter_mut() {
        // Find the type of button
        let Some((_, is_shuffle, is_draw)) = button_types
            .iter()
            .find(|(btn_entity, _, _)| *btn_entity == entity)
        else {
            continue;
        };

        match *interaction {
            Interaction::Pressed => {
                *bg_color = button_colors::PRESSED.into();
                border_color.0 = RED.into();

                if is_shuffle.is_some() {
                    ew_shuffle.write(DeckShuffle {
                        deck_entity,
                        duration: 8,
                    });
                } else if is_draw.is_some() {
                    ew_draw.write(DrawToHand {
                        deck_entity,
                        num_cards: 5,
                        player: 1,
                    });
                }
            }
            Interaction::Hovered => {
                *bg_color = button_colors::HOVERED.into();
                border_color.0 = WHITE.into();
            }
            Interaction::None => {
                *bg_color = button_colors::NORMAL.into();
                border_color.0 = BLACK.into();
            }
        }
    }
}

/// Set up the UI with shuffle and draw buttons
fn setup_ui(mut commands: Commands) {
    // Parent UI container
    commands
        .spawn((
            Name::new("Card UI"),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(20.0),
                left: Val::Percent(50.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(20.0),
                ..default()
            },
        ))
        .with_children(|child_builder| {
            // Shuffle button
            child_builder
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(350.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(BLACK.into()),
                    BorderRadius::MAX,
                    BackgroundColor(button_colors::NORMAL),
                    ButtonShuffleDeck,
                ))
                .with_children(|child_builder| {
                    child_builder.spawn((
                        Text::new("Shuffle deck"),
                        TextFont {
                            font_size: 33.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));
                });

            // Draw button
            child_builder
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(BLACK.into()),
                    BorderRadius::MAX,
                    BackgroundColor(button_colors::NORMAL),
                    ButtonDrawHand,
                ))
                .with_children(|child_builder| {
                    // Draw button text
                    child_builder.spawn((
                        Text::new("Draw hand"),
                        TextFont {
                            font_size: 33.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));
                });
        });
}
