use crate::{
    logic::{GamePhase, GamePhaseState, GameState, Mood, ResourceType},
    thoughts::ThoughtGeneratedEvent,
};
use bevy::{color::palettes::css::GREEN, prelude::*, text::TextBounds};

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpdateThoughtsEvent>()
            .add_systems(Startup, setup_ui)
            .add_systems(
                Update,
                (
                    update_phase_display,
                    update_time_display,
                    update_mood_display,
                    update_resource_bars,
                    update_character_thoughts,
                    handle_new_thought_generated,
                ),
            );
    }
}

// Event for updating character thoughts
#[derive(Event)]
pub struct UpdateThoughtsEvent {
    pub text: String,
    pub clear_after_seconds: Option<f32>, // Optional auto-clear timer
}

impl UpdateThoughtsEvent {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            clear_after_seconds: None,
        }
    }

    pub fn with_auto_clear(text: impl Into<String>, seconds: f32) -> Self {
        Self {
            text: text.into(),
            clear_after_seconds: Some(seconds),
        }
    }

    pub fn clear() -> Self {
        Self {
            text: String::new(),
            clear_after_seconds: None,
        }
    }
}

// UI Color scheme
pub struct UIColors;

impl UIColors {
    pub const BACKGROUND: Color = Color::srgba(0.1, 0.1, 0.15, 0.9);
    pub const THOUGHTS_BACKGROUND: Color = Color::srgba(0.15, 0.1, 0.2, 0.95);
    pub const TEXT: Color = Color::srgb(0.9, 0.9, 0.9);
    pub const TEXT_DIM: Color = Color::srgb(0.6, 0.6, 0.6);
    pub const THOUGHTS_TEXT: Color = Color::srgb(0.95, 0.9, 1.0);
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

    // Phase colors
    pub const PHASE_DRAW: Color = Color::srgb(0.3, 0.8, 0.9);
    pub const PHASE_SELECT: Color = Color::srgb(0.9, 0.7, 0.3);
    pub const PHASE_ACTION: Color = Color::srgb(0.7, 0.9, 0.4);
    pub const PHASE_CUTSCENE: Color = Color::srgb(0.9, 0.4, 0.8);

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

    pub fn phase_color(phase: &GamePhase) -> Color {
        match phase {
            GamePhase::CardDraw => Self::PHASE_DRAW,
            GamePhase::CardSelection => Self::PHASE_SELECT,
            GamePhase::CharacterAction => Self::PHASE_ACTION,
            GamePhase::VisualNovelCutscene => Self::PHASE_CUTSCENE,
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
pub struct PhaseDisplay;

#[derive(Component)]
pub struct TurnDisplay;

#[derive(Component)]
pub struct TimeDisplay;

#[derive(Component)]
pub struct DayDisplay;

#[derive(Component)]
pub struct MoodDisplay;

#[derive(Component)]
pub struct CharacterThoughts {
    pub clear_timer: Option<Timer>,
}

impl Default for CharacterThoughts {
    fn default() -> Self {
        Self { clear_timer: None }
    }
}

#[derive(Component)]
pub struct ResourceBar {
    pub resource_type: ResourceType,
}

#[derive(Component)]
pub struct ResourceBarFill;

pub const LAYER_UI: usize = 0;

// Setup the UI layout
fn setup_ui(mut commands: Commands) {
    // Main UI Root - fullscreen container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                ..default()
            },
            Name::new("UI ROOT"),
        ))
        .with_children(|root| {
            // Top Center - Character thoughts panel
            root.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexStart,
                    padding: UiRect::all(Val::Px(16.0)),
                    ..default()
                },
                Name::new("Top Container"),
            ))
            .with_children(|top_container| {
                // Character thoughts panel
                top_container
                    .spawn((
                        Node {
                            width: Val::Px(600.0),
                            min_height: Val::Px(80.0),
                            max_height: Val::Px(150.0),
                            padding: UiRect::all(Val::Px(16.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        // BackgroundColor::from(UIColors::THOUGHTS_BACKGROUND),
                        // BorderColor(UIColors::ACCENT.with_alpha(0.3)),
                        Name::new("Character Thoughts Panel"),
                    ))
                    .with_children(|thoughts_panel| {
                        thoughts_panel.spawn((
                            Text::new("What am I thinking about..."),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(UIColors::THOUGHTS_TEXT),
                            Node {
                                flex_wrap: FlexWrap::Wrap,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            CharacterThoughts::default(),
                        ));
                    });
            });

            // Left Side - Game state panel
            root.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    ..default()
                },
                Name::new("Left Container"),
            ))
            .with_children(|left_container| {
                // Game state panel
                left_container
                    .spawn((
                        Node {
                            width: Val::Px(280.0),
                            height: Val::Auto,
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(Val::Px(16.0)),
                            margin: UiRect::all(Val::Px(8.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        BackgroundColor::from(UIColors::BACKGROUND),
                        BorderColor(UIColors::ACCENT.with_alpha(0.2)),
                        Name::new("Game State Panel"),
                    ))
                    .with_children(|panel| {
                        // Game phase and turn display
                        panel
                            .spawn(Node {
                                margin: UiRect::bottom(Val::Px(16.0)),
                                flex_direction: FlexDirection::Column,
                                ..default()
                            })
                            .with_children(|phase_container| {
                                phase_container.spawn((
                                    Text::new("Phase: Card Draw"),
                                    TextFont {
                                        font_size: 16.0,
                                        ..default()
                                    },
                                    TextColor(UIColors::PHASE_DRAW),
                                    PhaseDisplay,
                                ));

                                phase_container.spawn((
                                    Text::new("Turn 1"),
                                    TextFont {
                                        font_size: 12.0,
                                        ..default()
                                    },
                                    TextColor(UIColors::TEXT_DIM),
                                    Node {
                                        margin: UiRect::top(Val::Px(2.0)),
                                        ..default()
                                    },
                                    TurnDisplay,
                                ));
                            });

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
                                                width: Val::Px(240.0),
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

                        // Time and day display at bottom
                        panel
                            .spawn(Node {
                                margin: UiRect::top(Val::Px(24.0)),
                                flex_direction: FlexDirection::Column,
                                ..default()
                            })
                            .with_children(|time_container| {
                                time_container.spawn((
                                    Text::new("10:00"),
                                    TextFont {
                                        font_size: 20.0,
                                        ..default()
                                    },
                                    TextColor(UIColors::TEXT),
                                    TimeDisplay,
                                ));

                                time_container.spawn((
                                    Text::new("Day 1"),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(UIColors::TEXT_DIM),
                                    DayDisplay,
                                ));
                            });
                    });
            });
        });
}

// Update character thoughts based on events
fn update_character_thoughts(
    mut thoughts_query: Query<(&mut Text, &mut CharacterThoughts)>,
    mut thoughts_events: EventReader<UpdateThoughtsEvent>,
    time: Res<Time>,
) {
    // Handle new thought events
    for event in thoughts_events.read() {
        for (mut text, mut thoughts) in &mut thoughts_query {
            *text = Text::new(&event.text);

            // Set up auto-clear timer if specified
            if let Some(clear_seconds) = event.clear_after_seconds {
                thoughts.clear_timer = Some(Timer::from_seconds(clear_seconds, TimerMode::Once));
            } else {
                thoughts.clear_timer = None;
            }
        }
    }

    // Handle auto-clear timers
    for (mut text, mut thoughts) in &mut thoughts_query {
        if let Some(ref mut timer) = thoughts.clear_timer {
            timer.tick(time.delta());
            if timer.finished() {
                *text = Text::new("");
                thoughts.clear_timer = None;
            }
        }
    }
}

// Update phase and turn display
fn update_phase_display(
    mut phase_query: Query<(&mut Text, &mut TextColor), (With<PhaseDisplay>, Without<TurnDisplay>)>,
    mut turn_query: Query<&mut Text, (With<TurnDisplay>, Without<PhaseDisplay>)>,
    phase_state: Res<GamePhaseState>,
) {
    if phase_state.is_changed() {
        // Update phase display
        for (mut text, mut text_color) in &mut phase_query {
            let phase_str = match phase_state.current_phase {
                GamePhase::CardDraw => "Card Draw",
                GamePhase::CardSelection => "Card Selection",
                GamePhase::CharacterAction => "Character Action",
                GamePhase::VisualNovelCutscene => "Cutscene",
            };
            *text = Text::new(format!("Phase: {phase_str}"));
            *text_color = TextColor(UIColors::phase_color(&phase_state.current_phase));
        }

        // Update turn display
        for mut text in &mut turn_query {
            *text = Text::new(format!("Turn {}", phase_state.turn_number));
        }
    }
}

// Update time and day display
fn update_time_display(
    mut time_query: Query<&mut Text, (With<TimeDisplay>, Without<DayDisplay>)>,
    mut day_query: Query<&mut Text, (With<DayDisplay>, Without<TimeDisplay>)>,
    game_state: Res<GameState>,
) {
    if game_state.is_changed() {
        for mut text in &mut time_query {
            *text = Text::new(game_state.get_time_string());
        }
        for mut text in &mut day_query {
            *text = Text::new(game_state.get_day_string());
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

fn handle_new_thought_generated(
    mut er_though_generated: EventReader<ThoughtGeneratedEvent>,
    mut ew_update_thoughts_ui: EventWriter<UpdateThoughtsEvent>,
) {
    for response in er_though_generated.read() {
        ew_update_thoughts_ui.write(UpdateThoughtsEvent::new(response.text.clone()));
    }
}
