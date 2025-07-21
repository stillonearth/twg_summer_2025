use avian2d::prelude::*;
use bevy::prelude::*;

const MOVE_SPEED: f32 = 200.;

#[derive(Default, Clone, Component)]
pub struct PlayerMarker;

pub fn move_player(
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
