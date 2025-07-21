use avian2d::prelude::Collider;
use bevy::color::Color;
use bevy::prelude::*;

use crate::collisions::calculate_distance_to_collider;
use crate::game_objects::WallProperties;
use crate::player::PlayerMarker;

pub fn debug_draw_system(
    mut gizmos: Gizmos,
    player_query: Query<&Transform, With<PlayerMarker>>,
    collider_query: Query<(Entity, &GlobalTransform, &Collider), Without<PlayerMarker>>,
    wall_properties_query: Query<&WallProperties>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_pos = player_transform.translation.truncate();

        // Draw interaction range circles
        gizmos.circle_2d(player_pos, 100.0, Color::srgba(0.0, 1.0, 0.0, 0.3));
        gizmos.circle_2d(player_pos, 50.0, Color::srgba(1.0, 1.0, 0.0, 0.5));
        gizmos.circle_2d(player_pos, 20.0, Color::srgba(1.0, 0.0, 0.0, 0.7));

        // Draw lines to nearby objects
        for (entity, wall_transform, collider) in collider_query.iter() {
            // Only draw lines to entities that have wall properties
            if wall_properties_query.get(entity).is_ok() {
                let wall_pos = wall_transform.translation().truncate();
                let distance = calculate_distance_to_collider(player_pos, wall_transform, collider);

                if distance < 100.0 {
                    let color = if distance < 20.0 {
                        Color::srgb(1.0, 0.0, 0.0) // Red for very close
                    } else if distance < 50.0 {
                        Color::srgb(1.0, 1.0, 0.0) // Yellow for close
                    } else {
                        Color::srgb(0.0, 1.0, 0.0) // Green for far
                    };

                    gizmos.line_2d(player_pos, wall_pos, color);
                }
            }
        }
    }
}
