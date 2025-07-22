use avian2d::prelude::*;
use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_ecs_tiled::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
mod collisions;
mod debug;
mod game_objects;
mod player;
mod sprites;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TiledMapPlugin::default())
        .add_plugins(TiledPhysicsPlugin::<TiledPhysicsAvianBackend>::default())
        .add_plugins(PhysicsPlugins::default().with_length_unit(100.0))
        .add_plugins((
            EguiPlugin {
                enable_multipass_for_primary_context: true,
            },
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        ))
        .register_type::<game_objects::WallProperties>()
        .register_type::<game_objects::WallType>()
        .add_systems(Startup, startup) // Remove sprites::spawn_character_sprite from here
        .add_systems(
            Update,
            (
                player::move_player,
                collisions::check_nearest_object,
                debug::debug_draw_system,
                sprites::animate_sprite,
                sprites::update_animation_indices,
            ),
        )
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let map_handle: Handle<TiledMap> = asset_server.load("my_room.tmx");
    commands
        .spawn((TiledMapHandle(map_handle), TilemapAnchor::Center))
        .observe(
            |_: Trigger<TiledMapCreated>,
             commands: Commands,
             asset_server: Res<AssetServer>,
             texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>| {
                // Call the combined spawn function when the map is created
                sprites::spawn_character_sprite(commands, asset_server, texture_atlas_layouts);
            },
        )
        .observe(
            |trigger: Trigger<TiledColliderCreated>, mut commands: Commands| {
                commands.entity(trigger.entity).insert((RigidBody::Static,));
            },
        );
}
