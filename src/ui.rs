use bevy::prelude::*;

// Import the types from your game logic plugin
use crate::logic::{GameState, Mood, ResourceType};

pub struct HikikomoriUIPlugin;

impl Plugin for HikikomoriUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui).add_systems(
            Update,
            (
                update_time_display,
                update_mood_display,
                update_resource_bars,
            ),
        );
    }
}

// UI Color scheme
pub struct UIColors;

impl UIColors {
    pub const BACKGROUND: Color = Color::srgba(0.1, 0.1, 0.15, 0.9);
    pub const TEXT: Color = Color::srgb(0.9, 0.9, 0.9);
    pub const TEXT_DIM: Color = Color::srgb(0.6, 0.6, 0.6);
    pub const ACCENT: Color = Color::srgb(0.4, 0.7, 1.0);

    // Resource colors
    pub const SLEEP_GOOD: Color = Color::srgb(0.3, 0.6, 0.9);
    pub const SLEEP_BAD: Color = Color::srgb(0.6, 0.3, 0.9);
    pub const HEALTH_GOOD: Color = Color::srgb(0.4, 0.8, 0.4);
    pub const HEALTH_BAD: Color = Color::srgb(0.8, 0.3, 0.3);
    pub const MENTAL_GOOD: Color = Color::srgb(0.9, 0.8, 0.3);
    pub const MENTAL_BAD: Color = Color::srgb(0.5, 0.2, 0.5);
    pub const FOOD_GOOD: Color = Color::srgb(0.9, 0.6, 0.2);
    pub const FOOD_BAD: Color = Color::srgb(0.6, 0.4, 0.2);

    pub fn mood_color(mood: &Mood) -> Color {
        match mood {
            Mood::Depressed => Color::srgb(0.3, 0.3, 0.4),
            Mood::Anxious => Color::srgb(0.8, 0.6, 0.2),
            Mood::Tired => Color::srgb(0.5, 0.4, 0.6),
            Mood::Neutral => Color::srgb(0.6, 0.6, 0.6),
            Mood::Content => Color::srgb(0.4, 0.7, 0.5),
            Mood::Manic => Color::srgb(0.9, 0.3, 0.6),
        }
    }

    pub fn resource_color(value: f32, good_color: Color, bad_color: Color) -> Color {
        let ratio = (value / 100.0).clamp(0.0, 1.0);

        let bad = bad_color.to_srgba();
        let good = good_color.to_srgba();

        Color::srgb(
            bad.red + (good.red - bad.red) * ratio,
            bad.green + (good.green - bad.green) * ratio,
            bad.blue + (good.blue - bad.blue) * ratio,
        )
    }
}

// UI component markers
#[derive(Component)]
pub struct TimeDisplay;

#[derive(Component)]
pub struct MoodDisplay;

#[derive(Component)]
pub struct ResourceBar {
    pub resource_type: ResourceType,
}

#[derive(Component)]
pub struct ResourceBarFill;

pub const LAYER_UI: usize = 0;

// Setup the UI layout
fn setup_ui(mut commands: Commands) {
    // Root UI container
    commands
        .spawn((
            Node {
                width: Val::Px(230.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            Name::new("Game UI"),
        ))
        .with_children(|parent| {
            // Top-left panel: Resources and mood
            parent
                .spawn((
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Auto,
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(16.0)),
                        margin: UiRect::all(Val::Px(8.0)),
                        ..default()
                    },
                    BackgroundColor::from(UIColors::BACKGROUND),
                ))
                .with_children(|panel| {
                    // Mood display
                    panel
                        .spawn(Node {
                            margin: UiRect::bottom(Val::Px(16.0)),
                            ..default()
                        })
                        .with_children(|mood_container| {
                            mood_container.spawn((
                                Text::new("Mood: Neutral"),
                                TextFont {
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(UIColors::TEXT),
                                MoodDisplay,
                            ));
                        });

                    // Resource bars
                    let resources = [
                        ("Sleep", ResourceType::Sleep),
                        ("Health", ResourceType::Health),
                        ("Mental", ResourceType::Mental),
                        ("Food", ResourceType::Food),
                    ];

                    for (label, resource_type) in resources {
                        panel
                            .spawn(Node {
                                margin: UiRect::bottom(Val::Px(8.0)),
                                display: Display::Block,
                                ..default()
                            })
                            .with_children(|resource_container| {
                                // Label
                                resource_container.spawn((
                                    Text::new(label),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(UIColors::TEXT_DIM),
                                ));

                                // Bar background
                                resource_container
                                    .spawn((
                                        Node {
                                            width: Val::Px(220.0),
                                            height: Val::Px(16.0),
                                            margin: UiRect::top(Val::Px(4.0)),
                                            ..default()
                                        },
                                        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                                        ResourceBar { resource_type },
                                    ))
                                    .with_children(|bar| {
                                        // Bar fill
                                        bar.spawn((
                                            Node {
                                                width: Val::Percent(70.0), // Will be updated
                                                height: Val::Percent(100.0),
                                                ..default()
                                            },
                                            BackgroundColor(UIColors::ACCENT),
                                            ResourceBarFill,
                                        ));
                                    });
                            });
                    }
                });

            // Time display panel (uncommented and positioned)
            parent
                .spawn((
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Auto,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexEnd,
                        padding: UiRect::all(Val::Px(16.0)),
                        margin: UiRect::all(Val::Px(8.0)),
                        ..default()
                    },
                    BackgroundColor(UIColors::BACKGROUND),
                ))
                .with_children(|panel| {
                    panel.spawn((
                        Text::new("10:00"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(UIColors::TEXT),
                        TimeDisplay,
                    ));

                    panel.spawn((
                        Text::new("Day 1"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(UIColors::TEXT_DIM),
                    ));
                });
        });
}

// Update time display
fn update_time_display(mut query: Query<&mut Text, With<TimeDisplay>>, game_state: Res<GameState>) {
    if game_state.is_changed() {
        for mut text in &mut query {
            *text = Text::new(game_state.get_time_string());
        }
    }
}

// Update mood display
fn update_mood_display(
    mut query: Query<(&mut Text, &mut TextColor), With<MoodDisplay>>,
    game_state: Res<GameState>,
) {
    if game_state.is_changed() {
        for (mut text, mut text_color) in &mut query {
            let mood_str = match game_state.current_mood {
                Mood::Depressed => "Depressed",
                Mood::Anxious => "Anxious",
                Mood::Tired => "Tired",
                Mood::Neutral => "Neutral",
                Mood::Content => "Content",
                Mood::Manic => "Manic",
            };
            *text = Text::new(format!("Mood: {mood_str}"));
            *text_color = TextColor(UIColors::mood_color(&game_state.current_mood));
        }
    }
}

// Update resource bars
fn update_resource_bars(
    mut fill_query: Query<
        (&mut Node, &mut BackgroundColor),
        (With<ResourceBarFill>, Without<ResourceBar>),
    >,
    bar_query: Query<(&ResourceBar, &Children)>,
    game_state: Res<GameState>,
) {
    if game_state.is_changed() {
        for (resource_bar, children) in &bar_query {
            let resource_value = game_state.get_resource_value(resource_bar.resource_type);

            let (good_color, bad_color) = match resource_bar.resource_type {
                ResourceType::Sleep => (UIColors::SLEEP_GOOD, UIColors::SLEEP_BAD),
                ResourceType::Health => (UIColors::HEALTH_GOOD, UIColors::HEALTH_BAD),
                ResourceType::Mental => (UIColors::MENTAL_GOOD, UIColors::MENTAL_BAD),
                ResourceType::Food => (UIColors::FOOD_GOOD, UIColors::FOOD_BAD),
            };

            // Update fill bar for this resource
            for child in children.iter() {
                if let Ok((mut style, mut bg_color)) = fill_query.get_mut(child) {
                    style.width = Val::Percent(resource_value);
                    bg_color.0 = UIColors::resource_color(resource_value, good_color, bad_color);
                }
            }
        }
    }
}
