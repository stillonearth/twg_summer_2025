use avian2d::prelude::*;
use bevy::prelude::*;

use crate::sprites::{
    AnimatedCharacterSprite, AnimationDirection, AnimationType, CharacterAnimation,
};

const MOVE_SPEED: f32 = 200.;

#[derive(Default, Clone, Component)]
pub struct PlayerMarker;

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut LinearVelocity, With<PlayerMarker>>,
    mut animation_query: Query<&mut CharacterAnimation, With<AnimatedCharacterSprite>>,
) {
    // Movement logic using arrow keys
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

    // Animation logic using WASD
    for mut character_animation in &mut animation_query {
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            character_animation.direction = AnimationDirection::Up;
            character_animation.animation_type = AnimationType::Walk;
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            character_animation.direction = AnimationDirection::Down;
            character_animation.animation_type = AnimationType::Walk;
        } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
            character_animation.direction = AnimationDirection::Left;
            character_animation.animation_type = AnimationType::Walk;
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            character_animation.direction = AnimationDirection::Right;
            character_animation.animation_type = AnimationType::Walk;
        } else {
            character_animation.animation_type = AnimationType::Stand;
        }
    }
}
