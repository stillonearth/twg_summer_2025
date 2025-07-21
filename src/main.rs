use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

const MOVE_SPEED: f32 = 200.;
const GRAVITY_SCALE: f32 = 10.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TiledMapPlugin::default())
        .add_plugins(TiledPhysicsPlugin::<TiledPhysicsAvianBackend>::default())
        .add_plugins(PhysicsPlugins::default().with_length_unit(100.0))
        .add_plugins(PhysicsDebugPlugin::default())
        .add_systems(Startup, startup)
        .add_systems(Update, move_player)
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn a Bevy 2D camera
    commands.spawn(Camera2d);

    commands.spawn(Text(String::from(
        "Move the ball using arrow keys or try to rotate the map!",
    )));

    // Load a map asset and retrieve the corresponding handle
    let map_handle: Handle<TiledMap> = asset_server.load("my_room.tmx");

    // Spawn a new entity with this handle
    commands
        .spawn((TiledMapHandle(map_handle), TilemapAnchor::Center))
        // Wait for map loading to complete and spawn a simple player-controlled object
        .observe(|_: Trigger<TiledMapCreated>, mut commands: Commands| {
            commands.spawn((
                RigidBody::Dynamic,
                PlayerMarker,
                Name::new("PlayerControlledObject (Avian2D physics)"),
                Collider::circle(10.),
                Transform::from_xyz(0., -50., 0.),
            ));
        })
        // Automatically insert a `RigidBody::Static` component on all the colliders entities from the map
        .observe(
            |trigger: Trigger<TiledColliderCreated>, mut commands: Commands| {
                commands.entity(trigger.entity).insert(RigidBody::Static);
            },
        );
}

// A 'player' marker component
#[derive(Default, Clone, Component)]
pub struct PlayerMarker;

// A simplistic controller
fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut LinearVelocity, With<PlayerMarker>>,
) {
    for mut rb_vel in player.iter_mut() {
        let mut direction = Vec2::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction += Vec2::new(1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction -= Vec2::new(1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::ArrowUp) {
            direction += Vec2::new(0.0, 1.0);
        }

        if keyboard_input.pressed(KeyCode::ArrowDown) {
            direction -= Vec2::new(0.0, 1.0);
        }

        if direction != Vec2::ZERO {
            direction /= direction.length();
        }

        rb_vel.0 = direction * MOVE_SPEED;
    }
}
