use std::f32::consts::FRAC_PI_2;

use avian2d::prelude::*;
use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_defer::AsyncPlugin;
use bevy_ecs_tiled::prelude::*;
use bevy_hui::HuiPlugin;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_kira_audio::AudioPlugin;
use bevy_la_mesa::{LaMesaPlugin, LaMesaPluginSettings};
use bevy_llm::LLMPlugin;
use bevy_novel::{NovelBackground, NovelImage, NovelPlugin, NovelText};

use crate::{
    cards::{ActivityCard, ActivityCards, ActivityCardsHandle, CardSystemPlugin},
    cutscene::CutscenePlugin,
    cutscene_menu::GameMenuPlugin,
    logic::{CutsceneEndEvent, CutsceneStartEvent, GameLogicPlugin},
    main_menu::MainMenuPlugin,
    player::PlayerPlugin,
    thoughts::CharacterThoughtsPlugin,
    ui::GameUIPlugin,
};

mod cards;
mod cutscene;
mod cutscene_menu;
mod game_objects;
mod logic;
mod main_menu;
mod navigation;
mod player;
mod sprites;
mod thoughts;
mod ui;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    MainMenu,
    Game,
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        IsDefaultUiCamera,
        Camera {
            order: 2,
            ..default()
        },
    ));

    commands.spawn((
        Name::new("Camera 3d"),
        Camera3d::default(),
        Camera {
            order: 1,
            ..default()
        },
        Transform::from_xyz(0.0, 25.0, 0.0)
            .looking_at(Vec3::ZERO, Vec3::Y)
            .with_rotation(Quat::from_rotation_z(0.0))
            .with_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
        MeshPickingCamera,
    ));

    let activity_cards_handle = ActivityCardsHandle(asset_server.load("cards.json"));
    commands.insert_resource(activity_cards_handle);
}

fn startup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut q_cameras: Query<(Entity, &mut Camera, &Camera2d)>,
) {
    let map_handle: Handle<TiledMap> = asset_server.load("my_room.tmx");
    commands
        .spawn((TiledMapHandle(map_handle), TilemapAnchor::Center))
        .observe(
            |_: Trigger<TiledMapCreated>,
             commands: Commands,
             asset_server: Res<AssetServer>,
             texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>| {
                player::spawn_player_sprite(commands, asset_server, texture_atlas_layouts);
            },
        )
        .observe(
            |trigger: Trigger<TiledColliderCreated>, mut commands: Commands| {
                commands.entity(trigger.entity).insert((RigidBody::Static,));
            },
        );

    for (_, mut camera, _) in q_cameras.iter_mut() {
        camera.order = 0;
    }
}

fn handle_cutscene_start(
    mut commands: Commands,
    mut er_cutscene_start: EventReader<CutsceneStartEvent>,
    q_tiled_map_markers: Query<(Entity, &TiledMapMarker)>,
) {
    for _ in er_cutscene_start.read() {
        for (entity, _) in q_tiled_map_markers.iter() {
            commands.entity(entity).insert(Visibility::Hidden);
        }
    }
}

fn handle_cutscene_end(
    mut commands: Commands,
    mut er_cutscene_end: EventReader<CutsceneEndEvent>,
    q_tiled_map_markers: Query<(Entity, &TiledMapMarker)>,
    mut q_novel_ui: ParamSet<(
        Query<(Entity, &mut Visibility, &NovelText)>,
        Query<(Entity, &mut Visibility, &NovelBackground)>,
        Query<(Entity, &mut Visibility, &NovelImage)>,
    )>,
) {
    for _ in er_cutscene_end.read() {
        for (entity, _) in q_tiled_map_markers.iter() {
            commands.entity(entity).insert(Visibility::Inherited);
        }
        for (_, mut visibility, _) in q_novel_ui.p0().iter_mut() {
            *visibility = Visibility::Hidden;
        }
        for (_, mut visibility, _) in q_novel_ui.p1().iter_mut() {
            *visibility = Visibility::Hidden;
        }
        for (_, mut visibility, _) in q_novel_ui.p2().iter_mut() {
            *visibility = Visibility::Hidden;
        }
    }
}

fn main() {
    App::new()
        .insert_resource(LaMesaPluginSettings { num_players: 1 })
        .insert_resource(MeshPickingSettings {
            require_markers: true,
            ray_cast_visibility: RayCastVisibility::Visible,
        })
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
            MeshPickingPlugin,
            LaMesaPlugin::<ActivityCard>::default(),
            game_objects::GameObjectsPlugin,
            TiledMapPlugin::default(),
            TiledPhysicsPlugin::<TiledPhysicsAvianBackend>::default(),
            PhysicsPlugins::default().with_length_unit(100.0),
            EguiPlugin {
                enable_multipass_for_primary_context: true,
            },
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
            navigation::NavigationGridPlugin {},
            JsonAssetPlugin::<ActivityCards>::new(&["json"]),
            AsyncPlugin::default_settings(),
            LLMPlugin,
            HuiPlugin,
            NovelPlugin {},
        ))
        .add_plugins((
            CharacterThoughtsPlugin,
            AudioPlugin,
            CutscenePlugin,
            CardSystemPlugin,
            GameLogicPlugin,
            GameUIPlugin,
            PlayerPlugin,
            GameMenuPlugin,
            MainMenuPlugin,
        ))
        .add_systems(Startup, startup)
        .add_systems(OnEnter(AppState::Game), startup_game)
        .add_systems(
            Update,
            (
                sprites::animate_sprite,
                sprites::update_animation_indices,
                handle_cutscene_start,
                handle_cutscene_end,
            ),
        )
        .init_state::<AppState>()
        .run();
}
