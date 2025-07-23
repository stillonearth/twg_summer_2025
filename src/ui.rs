use bevy::prelude::*;

pub struct HikikomoriUIPlugin;

impl Plugin for HikikomoriUIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameState>()
            .add_systems(Startup, setup_ui)
            .add_systems(
                Update,
                (
                    update_time_display,
                    update_mood_display,
                    update_resource_bars,
                    handle_ui_interactions,
                ),
            );
    }
}

// Core game state resource
#[derive(Resource)]
pub struct GameState {
    // Time
    pub current_hour: f32, // 0.0-24.0
    pub current_day: u32,
    pub time_speed: f32, // Time multiplier

    // Resources
    pub sleep: f32,         // 0.0-100.0
    pub health: f32,        // 0.0-100.0
    pub mental_health: f32, // 0.0-100.0
    pub food: f32,          // 0.0-100.0

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

impl GameState {
    pub fn update_time(&mut self, delta_time: f32) {
        self.current_hour += delta_time * self.time_speed * (1.0 / 3600.0); // Convert seconds to hours

        if self.current_hour >= 24.0 {
            self.current_hour -= 24.0;
            self.current_day += 1;
        }

        self.time_of_day = match self.current_hour {
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
        };

        self.current_mood = self.calculate_mood();
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

        // Extract RGB components properly
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

#[derive(Debug, Clone, Copy)]
pub enum ResourceType {
    Sleep,
    Health,
    Mental,
    Food,
}

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
            // RenderLayers::layer(LAYER_UI),
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

            // Top-right panel: Time display
            // parent
            //     .spawn((
            //         Node {
            //             width: Val::Px(200.0),
            //             height: Val::Auto,
            //             flex_direction: FlexDirection::Column,
            //             align_items: AlignItems::FlexEnd,
            //             padding: UiRect::all(Val::Px(16.0)),
            //             margin: UiRect::all(Val::Px(8.0)),
            //             ..default()
            //         },
            //         // BackgroundColor(UIColors::BACKGROUND.into()),
            //     ))
            //     .with_children(|panel| {
            //         panel.spawn((
            //             Text::new("10:00"),
            //             TextFont {
            //                 font_size: 24.0,
            //                 ..default()
            //             },
            //             TextColor(UIColors::TEXT),
            //             TimeDisplay,
            //         ));

            //         panel.spawn((
            //             Text::new("Day 1"),
            //             TextFont {
            //                 font_size: 16.0,
            //                 ..default()
            //             },
            //             TextColor(UIColors::TEXT_DIM),
            //         ));
            //     });
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
            let resource_value = match resource_bar.resource_type {
                ResourceType::Sleep => game_state.sleep,
                ResourceType::Health => game_state.health,
                ResourceType::Mental => game_state.mental_health,
                ResourceType::Food => game_state.food,
            };

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

// Handle UI interactions (placeholder for future expansion)
fn handle_ui_interactions() {
    // Future: Handle clicking on resource bars for details
    // Future: Handle time acceleration controls
    // Future: Handle mood-based UI effects
}

// Public API for other systems to update game state
impl GameState {
    pub fn modify_resource(&mut self, resource_type: ResourceType, amount: f32) {
        let resource = match resource_type {
            ResourceType::Sleep => &mut self.sleep,
            ResourceType::Health => &mut self.health,
            ResourceType::Mental => &mut self.mental_health,
            ResourceType::Food => &mut self.food,
        };

        *resource = (*resource + amount).clamp(0.0, 100.0);
        self.current_mood = self.calculate_mood();
    }

    pub fn set_time_speed(&mut self, speed: f32) {
        self.time_speed = speed.max(0.0);
    }
}

// Example usage system (remove this in actual game)
#[allow(dead_code)]
pub fn example_game_loop(
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
) {
    // Update time
    game_state.update_time(time.delta_secs());

    // Example input handling
    if input.just_pressed(KeyCode::KeyY) {
        let speed = if game_state.time_speed == 1.0 {
            10.0
        } else {
            1.0
        };
        game_state.set_time_speed(speed);
    }

    // Simulate resource decay over time
    let decay_rate = time.delta_secs() * 0.1; // Very slow decay for demo
    game_state.modify_resource(ResourceType::Food, -decay_rate);
    game_state.modify_resource(ResourceType::Sleep, -decay_rate * 0.5);
}
