use avian2d::prelude::*;
use bevy::{input::common_conditions::input_toggle_active, prelude::*, render::view::RenderLayers};
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_defer::AsyncPlugin;
use bevy_ecs_tiled::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_la_mesa::{LaMesaPlugin, LaMesaPluginSettings};
use bevy_llm::LLMPlugin;

use crate::{
    cards::{ActivityCardsHandle, CardSystemPlugin},
    logic::GameLogicPlugin,
    sprites::LAYER_SPRITES,
    thoughts::CharacterThoughtsPlugin,
};

mod cards;
mod collisions;
mod debug;
mod game_objects;
mod logic;
mod navigation;
mod player;
mod sprites;
mod thoughts;
mod ui;

fn main() {
    App::new()
        .insert_resource(LaMesaPluginSettings { num_players: 1 })
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
            ui::GameUIPlugin,
            LaMesaPlugin::<cards::ActivityCard>::default(),
            game_objects::GameObjectsPlugin,
            TiledMapPlugin::default(),
            TiledPhysicsPlugin::<TiledPhysicsAvianBackend>::default(),
            PhysicsPlugins::default().with_length_unit(100.0),
            EguiPlugin {
                enable_multipass_for_primary_context: true,
            },
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
            navigation::NavigationGridPlugin {},
            JsonAssetPlugin::<cards::ActivityCards>::new(&["json"]),
            AsyncPlugin::default_settings(),
            CardSystemPlugin,
            GameLogicPlugin,
        ))
        .add_plugins((LLMPlugin, CharacterThoughtsPlugin))
        .add_systems(Startup, startup)
        .add_systems(
            Update,
            (
                player::move_player_from_command,
                player::move_player_along_path,
                collisions::check_nearest_object,
                // debug::debug_draw_system,
                sprites::animate_sprite,
                sprites::update_animation_indices,
            ),
        )
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, IsDefaultUiCamera));

    commands.spawn((
        Name::new("Camera 2d"),
        Camera2d,
        Camera {
            order: 1,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1000.0),
    ));

    commands.spawn((
        Name::new("Camera 3d"),
        Camera3d::default(),
        Camera {
            order: 2,
            ..default()
        },
        // Pickable::default(),
        Transform::from_xyz(0.0, 15.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

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

    let activity_cards_handle = ActivityCardsHandle(asset_server.load("cards.json"));
    commands.insert_resource(activity_cards_handle);
}
