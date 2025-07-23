use crate::navigation::{GridPos, MovePlayerCommand, TileSize};
use crate::sprites::{
    AnimatedCharacterSprite, AnimationDirection, AnimationType, CharacterAnimation,
};
use avian2d::prelude::*;
use bevy::prelude::*; // Add your navigation imports

const MOVE_SPEED: f32 = 200.;

#[derive(Default, Clone, Component)]
pub struct PlayerMarker;

#[derive(Component)]
pub struct PlayerMovement {
    pub path: Vec<GridPos>,
    pub current_target_index: usize,
    pub is_moving: bool,
}

impl Default for PlayerMovement {
    fn default() -> Self {
        Self {
            path: Vec::new(),
            current_target_index: 0,
            is_moving: false,
        }
    }
}

pub fn move_player_from_command(
    mut move_events: EventReader<MovePlayerCommand>,
    mut player_query: Query<&mut PlayerMovement, With<PlayerMarker>>,
) {
    for event in move_events.read() {
        if let Ok(mut player_movement) = player_query.get_single_mut() {
            // Skip the first position in the path (current position)
            let path = if event.path.len() > 1 {
                event.path[1..].to_vec()
            } else {
                event.path.clone()
            };

            player_movement.path = path;
            player_movement.current_target_index = 0;
            player_movement.is_moving = !player_movement.path.is_empty();

            println!(
                "Player received movement command with {} waypoints",
                player_movement.path.len()
            );
        }
    }
}

pub fn move_player_along_path(
    mut player_query: Query<
        (&mut Transform, &mut LinearVelocity, &mut PlayerMovement),
        With<PlayerMarker>,
    >,
    mut animation_query: Query<&mut CharacterAnimation, With<AnimatedCharacterSprite>>,
    tile_size: Res<TileSize>,
    time: Res<Time>,
) {
    for (mut transform, mut rb_vel, mut player_movement) in player_query.iter_mut() {
        if !player_movement.is_moving || player_movement.path.is_empty() {
            rb_vel.0 = Vec2::ZERO;
            // Set animation to stand when not moving
            for mut character_animation in &mut animation_query {
                character_animation.animation_type = AnimationType::Stand;
            }
            continue;
        }

        let current_target = player_movement.path[player_movement.current_target_index];
        let target_world_pos = Vec3::new(
            current_target.x as f32 * tile_size.0 + tile_size.0 / 2.0,
            current_target.y as f32 * tile_size.0 + tile_size.0 / 2.0,
            transform.translation.z, // Keep the same Z
        );

        let direction = (target_world_pos - transform.translation).truncate();
        let distance = direction.length();

        // Check if we've reached the current target
        if distance < 5.0 {
            // Small threshold for "reached"
            player_movement.current_target_index += 1;

            // Check if we've reached the end of the path
            if player_movement.current_target_index >= player_movement.path.len() {
                player_movement.is_moving = false;
                rb_vel.0 = Vec2::ZERO;

                // Set animation to stand when path is complete
                for mut character_animation in &mut animation_query {
                    character_animation.animation_type = AnimationType::Stand;
                }

                println!("Player reached destination!");
                continue;
            }
        }

        // Move towards the current target
        let normalized_direction = direction.normalize_or_zero();
        rb_vel.0 = normalized_direction * MOVE_SPEED;

        // Update animation based on movement direction
        for mut character_animation in &mut animation_query {
            character_animation.animation_type = AnimationType::Walk;

            // Determine animation direction based on movement
            if normalized_direction.x.abs() > normalized_direction.y.abs() {
                if normalized_direction.x > 0.0 {
                    character_animation.direction = AnimationDirection::Right;
                } else {
                    character_animation.direction = AnimationDirection::Left;
                }
            } else {
                if normalized_direction.y > 0.0 {
                    character_animation.direction = AnimationDirection::Up;
                } else {
                    character_animation.direction = AnimationDirection::Down;
                }
            }
        }
    }
}

// Your existing keyboard movement system
pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut LinearVelocity, &mut PlayerMovement), With<PlayerMarker>>,
    mut animation_query: Query<&mut CharacterAnimation, With<AnimatedCharacterSprite>>,
) {
    // Movement logic using arrow keys
    for (mut rb_vel, mut player_movement) in player.iter_mut() {
        let mut direction = Vec2::ZERO;
        let mut has_keyboard_input = false;

        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction += Vec2::new(1.0, 0.0);
            has_keyboard_input = true;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction -= Vec2::new(1.0, 0.0);
            has_keyboard_input = true;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            direction += Vec2::new(0.0, 1.0);
            has_keyboard_input = true;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            direction -= Vec2::new(0.0, 1.0);
            has_keyboard_input = true;
        }

        // If there's keyboard input, cancel pathfinding movement
        if has_keyboard_input {
            player_movement.is_moving = false;
            player_movement.path.clear();

            if direction != Vec2::ZERO {
                direction /= direction.length();
            }
            rb_vel.0 = direction * MOVE_SPEED;
        } else if !player_movement.is_moving {
            // Only stop if not pathfinding
            rb_vel.0 = Vec2::ZERO;
        }
    }

    // Animation logic using arrow keys (only if not pathfinding)
    for (_, player_movement) in player.iter() {
        if player_movement.is_moving {
            continue; // Skip keyboard animation if pathfinding
        }

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
}
