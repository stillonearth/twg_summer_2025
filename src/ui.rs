use crate::{
    cards::{CrisisLevel, Mood, ResourceType, StatusEffect},
    logic::{
        ActiveStatusEffect, CutsceneEndEvent, CutsceneStartEvent, GamePhase, GamePhaseState,
        GameState,
    },
    thoughts::ThoughtGeneratedEvent,
};
use bevy::prelude::*;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpdateThoughtsEvent>()
            .add_systems(Startup, setup_ui)
            .add_systems(
                Update,
                (update_displays, update_character_thoughts, handle_events),
            );
    }
}

#[derive(Event)]
pub struct UpdateThoughtsEvent {
    pub text: String,
}

impl UpdateThoughtsEvent {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

// Consolidated UI colors
pub struct UIColors;

impl UIColors {
    pub const BACKGROUND: Color = Color::srgba(0.1, 0.1, 0.15, 0.9);
    pub const TEXT: Color = Color::srgb(0.9, 0.9, 0.9);
    pub const TEXT_DIM: Color = Color::srgb(0.6, 0.6, 0.6);
    pub const THOUGHTS_TEXT: Color = Color::srgb(0.95, 0.9, 1.0);
    pub const ACCENT: Color = Color::srgb(0.4, 0.7, 1.0);

    const RESOURCE_COLORS: [(Color, Color); 4] = [
        (Color::srgb(0.3, 0.6, 0.9), Color::srgb(0.6, 0.3, 0.9)), // Sleep
        (Color::srgb(0.4, 0.8, 0.4), Color::srgb(0.8, 0.3, 0.3)), // Health
        (Color::srgb(0.9, 0.8, 0.3), Color::srgb(0.5, 0.2, 0.5)), // Mental
        (Color::srgb(0.9, 0.6, 0.2), Color::srgb(0.6, 0.4, 0.2)), // Food
    ];

    const PHASE_COLORS: [Color; 4] = [
        Color::srgb(0.3, 0.8, 0.9), // Draw
        Color::srgb(0.9, 0.7, 0.3), // Select
        Color::srgb(0.7, 0.9, 0.4), // Action
        Color::srgb(0.9, 0.4, 0.8), // TurnOver
    ];

    const MOOD_COLORS: [Color; 6] = [
        Color::srgb(0.3, 0.3, 0.4), // Depressed
        Color::srgb(0.8, 0.6, 0.2), // Anxious
        Color::srgb(0.5, 0.4, 0.6), // Tired
        Color::srgb(0.6, 0.6, 0.6), // Neutral
        Color::srgb(0.4, 0.7, 0.5), // Content
        Color::srgb(0.9, 0.3, 0.6), // Manic
    ];

    const CRISIS_COLORS: [Color; 5] = [
        Color::srgb(0.4, 0.7, 0.5), // None - Green
        Color::srgb(0.9, 0.8, 0.3), // Mild - Yellow
        Color::srgb(0.9, 0.6, 0.2), // Moderate - Orange
        Color::srgb(0.9, 0.4, 0.2), // Severe - Red-Orange
        Color::srgb(0.9, 0.2, 0.2), // Critical - Red
    ];

    const STATUS_EFFECT_COLORS: [Color; 12] = [
        Color::srgb(0.3, 0.4, 0.8), // Insomnia - Blue
        Color::srgb(0.8, 0.3, 0.3), // Sick - Red
        Color::srgb(0.4, 0.8, 0.4), // Motivated - Green
        Color::srgb(0.8, 0.6, 0.2), // Overwhelmed - Orange
        Color::srgb(0.8, 0.2, 0.8), // Addicted - Purple
        Color::srgb(0.6, 0.6, 0.3), // Exhausted - Brown
        Color::srgb(0.8, 0.8, 0.2), // Anxious - Yellow
        Color::srgb(0.4, 0.4, 0.6), // Depressed - Dark Blue
        Color::srgb(0.9, 0.3, 0.6), // Manic - Pink
        Color::srgb(0.5, 0.7, 0.5), // Stable - Light Green
        Color::srgb(0.3, 0.7, 0.9), // Focused - Cyan
        Color::srgb(0.9, 0.5, 0.2), // Hungry - Orange-Red
    ];

    pub fn mood_color(mood: &Mood) -> Color {
        Self::MOOD_COLORS[*mood as usize]
    }

    pub fn phase_color(phase: &GamePhase) -> Color {
        Self::PHASE_COLORS[*phase as usize]
    }

    pub fn crisis_color(crisis_level: &CrisisLevel) -> Color {
        Self::CRISIS_COLORS[*crisis_level as usize]
    }

    pub fn status_effect_color(effect: &StatusEffect) -> Color {
        let index = match effect {
            StatusEffect::Insomnia(_) => 0,
            StatusEffect::Sick(_) => 1,
            StatusEffect::Motivated(_) => 2,
            StatusEffect::Overwhelmed(_) => 3,
            StatusEffect::Addicted(_, _) => 4,
            StatusEffect::Exhausted(_) => 5,
            StatusEffect::Anxious(_) => 6,
            StatusEffect::Depressed(_) => 7,
            StatusEffect::Manic(_) => 8,
            StatusEffect::Stable(_) => 9,
            StatusEffect::Focused(_) => 10,
            StatusEffect::Hungry(_) => 11,
        };
        Self::STATUS_EFFECT_COLORS[index]
    }

    pub fn status_effect_name(effect: &StatusEffect) -> String {
        match effect {
            StatusEffect::Insomnia(_) => "Insomnia".to_string(),
            StatusEffect::Sick(_) => "Sick".to_string(),
            StatusEffect::Motivated(_) => "Motivated".to_string(),
            StatusEffect::Overwhelmed(_) => "Overwhelmed".to_string(),
            StatusEffect::Addicted(substance, _) => format!("Addicted ({})", substance),
            StatusEffect::Exhausted(_) => "Exhausted".to_string(),
            StatusEffect::Anxious(_) => "Anxious".to_string(),
            StatusEffect::Depressed(_) => "Depressed".to_string(),
            StatusEffect::Manic(_) => "Manic".to_string(),
            StatusEffect::Stable(_) => "Stable".to_string(),
            StatusEffect::Focused(_) => "Focused".to_string(),
            StatusEffect::Hungry(_) => "Hungry".to_string(),
        }
    }

    pub fn resource_colors(resource_type: ResourceType) -> (Color, Color) {
        Self::RESOURCE_COLORS[resource_type as usize]
    }

    pub fn lerp_color(value: f32, good: Color, bad: Color) -> Color {
        let t = (value / 100.0).clamp(0.0, 1.0);
        let bad_srgb = bad.to_srgba();
        let good_srgb = good.to_srgba();

        Color::srgb(
            bad_srgb.red + (good_srgb.red - bad_srgb.red) * t,
            bad_srgb.green + (good_srgb.green - bad_srgb.green) * t,
            bad_srgb.blue + (good_srgb.blue - bad_srgb.blue) * t,
        )
    }
}

// Component markers
#[derive(Component)]
pub struct UIRoot;

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
pub struct CrisisDisplay;

#[derive(Component)]
pub struct StatusEffectsPanel;

#[derive(Component)]
pub struct StatusEffectItem;

#[derive(Component, Default)]
pub struct CharacterThoughts {
    pub clear_timer: Option<Timer>,
}

#[derive(Component)]
pub struct AvatarImage;

#[derive(Component)]
pub struct ResourceBar {
    pub resource_type: ResourceType,
}

#[derive(Component)]
pub struct ResourceBarFill;

#[derive(Component)]
pub struct ResourceValueText {
    pub resource_type: ResourceType,
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_left_panel(&mut commands);
    spawn_thoughts_panel(&mut commands, &asset_server);
}

fn spawn_left_panel(commands: &mut Commands) {
    commands
        .spawn((
            UIRoot,
            Node {
                width: Val::Px(280.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                ..default()
            },
            Name::new("Left Panel"),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(16.0)),
                        margin: UiRect::all(Val::Px(8.0)),
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    BorderRadius::new(Val::Px(5.), Val::Px(5.), Val::Px(5.), Val::Px(5.)),
                    BackgroundColor::from(UIColors::BACKGROUND),
                    BorderColor(UIColors::ACCENT.with_alpha(0.2)),
                ))
                .with_children(|panel| {
                    spawn_text_section(
                        panel,
                        "Turn 1",
                        12.0,
                        UIColors::TEXT_DIM,
                        Some(TurnDisplay),
                    );

                    spawn_text_section(
                        panel,
                        "Phase: Card Draw",
                        16.0,
                        UIColors::PHASE_COLORS[0],
                        Some(PhaseDisplay),
                    );

                    spawn_text_section(
                        panel,
                        "Mood: Neutral",
                        16.0,
                        UIColors::TEXT,
                        Some(MoodDisplay),
                    );

                    spawn_text_section(
                        panel,
                        "Crisis: None",
                        16.0,
                        UIColors::CRISIS_COLORS[0],
                        Some(CrisisDisplay),
                    );

                    // Resource bars
                    for (i, &(label, resource_type)) in [
                        ("Sleep", ResourceType::Sleep),
                        ("Health", ResourceType::Health),
                        ("Mental", ResourceType::Mental),
                        ("Food", ResourceType::Food),
                    ]
                    .iter()
                    .enumerate()
                    {
                        spawn_resource_bar(panel, label, resource_type);
                    }

                    // Time displays
                    spawn_text_section(panel, "10:00", 20.0, UIColors::TEXT, Some(TimeDisplay));
                    spawn_text_section(panel, "Day 1", 14.0, UIColors::TEXT_DIM, Some(DayDisplay));

                    // Status Effects Section
                    panel
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                margin: UiRect::top(Val::Px(16.0)),

                                ..default()
                            },
                            StatusEffectsPanel,
                        ))
                        .with_children(|status_panel| {
                            // Status effects header
                            status_panel.spawn((
                                Text::new("Active Effects"),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(UIColors::TEXT),
                                Node {
                                    margin: UiRect::bottom(Val::Px(8.0)),
                                    padding: UiRect::top(Val::Px(20.0)),
                                    display: Display::Grid,
                                    ..default()
                                },
                            ));
                        });
                });
        });
}

fn spawn_thoughts_panel(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands
        .spawn((
            UIRoot,
            Node {
                width: Val::Auto,
                height: Val::Px(150.0),
                position_type: PositionType::Absolute,
                left: Val::Px(300.0),
                top: Val::Px(0.0),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                ..default()
            },
            Name::new("Thoughts Panel"),
        ))
        .with_children(|parent| {
            // Avatar image container
            parent
                .spawn(Node {
                    width: Val::Px(120.0),
                    height: Val::Px(120.0),
                    margin: UiRect::right(Val::Px(20.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                })
                // .insert(BorderRadius::all(Val::Px(60.0))) // Circular border
                // .insert(BorderColor(UIColors::ACCENT.with_alpha(0.3)))
                // .insert(BackgroundColor(UIColors::BACKGROUND.with_alpha(0.5)))
                .with_children(|avatar_container| {
                    avatar_container.spawn((
                        ImageNode::new(asset_server.load("avatars/avatar_anxious.png")),
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        // BorderRadius::all(Val::Px(58.0)), // Slightly smaller radius for the image
                        AvatarImage,
                    ));
                });

            // Thoughts text container
            parent
                .spawn(Node {
                    width: Val::Px(600.0),
                    min_height: Val::Px(80.0),
                    max_height: Val::Px(130.0),
                    padding: UiRect::all(Val::Px(16.0)),
                    justify_content: JustifyContent::Default,
                    align_items: AlignItems::Default,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                })
                // .insert(BorderRadius::all(Val::Px(8.0)))
                // .insert(BackgroundColor(UIColors::BACKGROUND.with_alpha(0.7)))
                // .insert(BorderColor(UIColors::ACCENT.with_alpha(0.2)))
                .with_children(|panel| {
                    panel.spawn((
                        Text::new("A new day to survive..."),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(UIColors::THOUGHTS_TEXT),
                        CharacterThoughts::default(),
                    ));
                });
        });
}

fn spawn_text_section<T: Component>(
    parent: &mut ChildSpawnerCommands,
    text: &str,
    font_size: f32,
    color: Color,
    marker: Option<T>,
) {
    let mut entity = parent.spawn((
        Text::new(text),
        TextFont {
            font_size,
            ..default()
        },
        TextColor(color),
        Node {
            margin: UiRect::bottom(Val::Px(8.0)),
            ..default()
        },
    ));

    if let Some(component) = marker {
        entity.insert(component);
    }
}

fn spawn_resource_bar(parent: &mut ChildSpawnerCommands, label: &str, resource_type: ResourceType) {
    parent
        .spawn(Node {
            margin: UiRect::bottom(Val::Px(8.0)),
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|container| {
            // Header row with label and value
            container
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    margin: UiRect::bottom(Val::Px(4.0)),
                    ..default()
                })
                .with_children(|header| {
                    header.spawn((
                        Text::new(label),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(UIColors::TEXT_DIM),
                    ));

                    header.spawn((
                        Text::new("70/100"),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(UIColors::TEXT),
                        ResourceValueText { resource_type },
                    ));
                });

            // Progress bar
            container
                .spawn((
                    Node {
                        width: Val::Px(240.0),
                        height: Val::Px(16.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    ResourceBar { resource_type },
                ))
                .with_children(|bar| {
                    bar.spawn((
                        Node {
                            width: Val::Percent(70.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(UIColors::ACCENT),
                        ResourceBarFill,
                    ));
                });
        });
}

// Consolidated update systems
fn update_displays(
    mut commands: Commands,
    mut text_queries: ParamSet<(
        Query<(&mut Text, &mut TextColor), (With<PhaseDisplay>, Without<TurnDisplay>)>,
        Query<&mut Text, (With<TurnDisplay>, Without<PhaseDisplay>)>,
        Query<&mut Text, (With<TimeDisplay>, Without<DayDisplay>)>,
        Query<&mut Text, (With<DayDisplay>, Without<TimeDisplay>)>,
        Query<(&mut Text, &mut TextColor), With<MoodDisplay>>,
        Query<(&mut Text, &mut TextColor), With<CrisisDisplay>>,
        Query<(&mut Text, &ResourceValueText)>,
    )>,
    mut fill_query: Query<
        (&mut Node, &mut BackgroundColor),
        (With<ResourceBarFill>, Without<ResourceBar>),
    >,
    bar_query: Query<(&ResourceBar, &Children)>,
    status_effects_panel_query: Query<(Entity, &Children), With<StatusEffectsPanel>>,
    status_effect_items_query: Query<Entity, With<StatusEffectItem>>,
    phase_state: Res<GamePhaseState>,
    game_state: Res<GameState>,
    asset_server: Res<AssetServer>,
    mut avatar_query: Query<&mut ImageNode, With<AvatarImage>>,
) {
    // Update phase display
    if phase_state.is_changed() {
        for (mut text, mut color) in text_queries.p0().iter_mut() {
            *text = Text::new(format!("Phase: {}", phase_state.get_phase_name()));
            *color = TextColor(UIColors::phase_color(&phase_state.current_phase));
        }

        for mut text in text_queries.p1().iter_mut() {
            *text = Text::new(format!("Turn {}", phase_state.turn_number));
        }
    }

    // Update game state displays
    if game_state.is_changed() {
        // Update avatar based on mood
        for mut image in avatar_query.iter_mut() {
            let avatar_path = match game_state.current_mood {
                Mood::Depressed => "avatars/avatar_depressed.png",
                Mood::Anxious => "avatars/avatar_anxious.png",
                Mood::Tired => "avatars/avatar_tired.png",
                Mood::Neutral => "avatars/avatar_neutral.png",
                Mood::Content => "avatars/avatar_content.png",
                Mood::Manic => "avatars/avatar_manic.png",
            };
            *image = ImageNode::new(asset_server.load(avatar_path));
        }

        // Time displays
        for mut text in text_queries.p2().iter_mut() {
            *text = Text::new(game_state.get_time_string());
        }
        for mut text in text_queries.p3().iter_mut() {
            *text = Text::new(game_state.get_day_string());
        }

        // Mood display
        for (mut text, mut color) in text_queries.p4().iter_mut() {
            let mood_name = [
                "Depressed",
                "Anxious",
                "Tired",
                "Neutral",
                "Content",
                "Manic",
            ][game_state.current_mood as usize];
            *text = Text::new(format!("Mood: {mood_name}"));
            *color = TextColor(UIColors::mood_color(&game_state.current_mood));
        }

        // Crisis display
        for (mut text, mut color) in text_queries.p5().iter_mut() {
            let crisis_name = match game_state.crisis_level {
                CrisisLevel::None => "None",
                CrisisLevel::Mild => "Mild",
                CrisisLevel::Moderate => "Moderate",
                CrisisLevel::Severe => "Severe",
                CrisisLevel::Critical => "Critical",
            };
            *text = Text::new(format!("Crisis: {crisis_name}"));
            *color = TextColor(UIColors::crisis_color(&game_state.crisis_level));
        }

        // Resource value text
        for (mut text, resource_value) in text_queries.p6().iter_mut() {
            let value = game_state.get_resource_value(resource_value.resource_type);
            *text = Text::new(format!("{value:.0}/100"));
        }

        // Resource bars
        for (resource_bar, children) in &bar_query {
            let value = game_state.get_resource_value(resource_bar.resource_type);
            let (good_color, bad_color) = UIColors::resource_colors(resource_bar.resource_type);

            for child in children.iter() {
                if let Ok((mut style, mut bg_color)) = fill_query.get_mut(child) {
                    style.width = Val::Percent(value);
                    bg_color.0 = UIColors::lerp_color(value, good_color, bad_color);
                }
            }
        }
    }

    // Update status effects display
    if game_state.is_changed() {
        update_status_effects_display(
            &mut commands,
            &status_effects_panel_query,
            &status_effect_items_query,
            &game_state,
        );
    }
}

fn update_status_effects_display(
    commands: &mut Commands,
    status_effects_panel_query: &Query<(Entity, &Children), With<StatusEffectsPanel>>,
    status_effect_items_query: &Query<Entity, With<StatusEffectItem>>,
    game_state: &GameState,
) {
    // Clear existing status effect items
    for entity in status_effect_items_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Find the status effects panel
    if let Ok((panel_entity, children)) = status_effects_panel_query.get_single() {
        // Find the panel container (skip the header)
        for child in children.iter() {
            commands.entity(child).with_children(|panel| {
                // Skip the header and add status effects
                if game_state.status_effects.is_empty() {
                    panel.spawn((
                        Text::new("No active effects"),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(UIColors::TEXT_DIM),
                        Node {
                            margin: UiRect::bottom(Val::Px(4.0)),
                            ..default()
                        },
                        StatusEffectItem,
                    ));
                } else {
                    for status_effect in &game_state.status_effects {
                        spawn_status_effect_item(panel, status_effect);
                    }
                }
            });
            break; // Only add to the first child (the panel container)
        }
    }
}

fn spawn_status_effect_item(parent: &mut ChildSpawnerCommands, status_effect: &ActiveStatusEffect) {
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                margin: UiRect::bottom(Val::Px(6.0)),
                padding: UiRect::all(Val::Px(6.0)),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BorderRadius::new(Val::Px(3.), Val::Px(3.), Val::Px(3.), Val::Px(3.)),
            BackgroundColor(UIColors::BACKGROUND.with_alpha(0.3)),
            BorderColor(UIColors::status_effect_color(&status_effect.effect).with_alpha(0.5)),
            StatusEffectItem,
        ))
        .with_children(|item| {
            // Effect name
            item.spawn((
                Text::new(UIColors::status_effect_name(&status_effect.effect)),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(UIColors::status_effect_color(&status_effect.effect)),
                Node {
                    margin: UiRect::bottom(Val::Px(2.0)),
                    ..default()
                },
            ));

            // Duration and intensity info
            let info_text = if status_effect.intensity != 1.0 {
                format!(
                    "{} turns • {:.0}%",
                    status_effect.remaining_duration,
                    status_effect.intensity * 100.0
                )
            } else {
                format!("{} turns", status_effect.remaining_duration)
            };

            item.spawn((
                Text::new(info_text),
                TextFont {
                    font_size: 9.0,
                    ..default()
                },
                TextColor(UIColors::TEXT_DIM),
            ));

            // Source info if available (shorter format for left panel)
            if !status_effect.source.is_empty() {
                item.spawn((
                    Text::new(format!("({})", status_effect.source)),
                    TextFont {
                        font_size: 8.0,
                        ..default()
                    },
                    TextColor(UIColors::TEXT_DIM.with_alpha(0.7)),
                ));
            }
        });
}

fn update_character_thoughts(
    mut thoughts_query: Query<(&mut Text, &mut CharacterThoughts)>,
    mut thoughts_events: EventReader<UpdateThoughtsEvent>,
    time: Res<Time>,
) {
    for event in thoughts_events.read() {
        for (mut text, _) in &mut thoughts_query {
            *text = Text::new(&event.text);
        }
    }

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

fn handle_events(
    mut commands: Commands,
    mut er_thought: EventReader<ThoughtGeneratedEvent>,
    mut er_cutscene_start: EventReader<CutsceneStartEvent>,
    mut er_cutscene_end: EventReader<CutsceneEndEvent>,
    mut ew_update_thoughts: EventWriter<UpdateThoughtsEvent>,
    q_ui_roots: Query<(Entity, &UIRoot)>,
) {
    // Handle thought events
    for event in er_thought.read() {
        ew_update_thoughts.write(UpdateThoughtsEvent::new(event.text.clone()));
    }

    // Handle cutscene events
    for _ in er_cutscene_start.read() {
        for (entity, _) in &q_ui_roots {
            commands.entity(entity).insert(Visibility::Hidden);
        }
    }

    for _ in er_cutscene_end.read() {
        for (entity, _) in &q_ui_roots {
            commands.entity(entity).insert(Visibility::Inherited);
        }
    }
}
