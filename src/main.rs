use avian2d::prelude::*;
use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_ecs_tiled::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

const MOVE_SPEED: f32 = 200.;

mod collisions;
mod debug;
mod game_objects;
mod player;

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
        .add_systems(Startup, startup)
        .add_systems(
            Update,
            (player::move_player, check_nearest_object, debug_draw_system),
        )
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(Text(String::from(
        "Move the ball using arrow keys! Press spacebar to check nearest wall properties.",
    )));
    let map_handle: Handle<TiledMap> = asset_server.load("my_room.tmx");
    commands
        .spawn((TiledMapHandle(map_handle), TilemapAnchor::Center))
        .observe(|_: Trigger<TiledMapCreated>, mut commands: Commands| {
            commands.spawn((
                RigidBody::Dynamic,
                PlayerMarker,
                Name::new("PlayerControlledObject (Avian2D physics)"),
                Collider::circle(10.),
                Transform::from_xyz(0., -50., 0.),
            ));
        })
        .observe(
            |trigger: Trigger<TiledColliderCreated>, mut commands: Commands| {
                // Add both the RigidBody and WallProperties when a collider is created
                commands.entity(trigger.entity).insert((
                    RigidBody::Static,
                    WallProperties {
                        name: "Wall".to_string(),
                        wall_type: WallType::Stone,
                    },
                ));
            },
        );
}
