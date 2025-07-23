use avian2d::prelude::Collider;
use bevy::prelude::*;

use crate::{game_objects::WallProperties, player::PlayerMarker};

pub fn check_nearest_object(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_query: Query<&GlobalTransform, With<PlayerMarker>>,
    collider_query: Query<(Entity, &ChildOf, &GlobalTransform, &Collider), Without<PlayerMarker>>,
    wall_properties_query: Query<&WallProperties>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Ok(player_transform) = player_query.get_single() {
            let player_pos = player_transform.translation().truncate();

            let nearest_by_distance = find_nearest_by_distance_separate_queries(
                &player_pos,
                &collider_query,
                &wall_properties_query,
            );

            let selected_wall = nearest_by_distance;

            match selected_wall {
                Some((wall, distance, method)) => {
                    println!("\n=== Nearest Wall Properties ===");
                    println!("Detection method: {method}");
                    println!("Name: {}", wall.name);
                    println!("Distance to surface: {distance:.2} units");

                    // Add interaction feedback based on distance
                    if distance < 20.0 {
                        println!("🔥 Very close - can interact!");
                    } else if distance < 50.0 {
                        println!("📍 Close - move closer to interact");
                    } else {
                        println!("👁️  Visible - too far to interact");
                    }
                    println!("===============================");
                }
                None => {
                    println!("No walls found within interaction range!");
                }
            }
        }
    }
}

pub fn find_nearest_by_distance_separate_queries(
    player_pos: &Vec2,
    collider_query: &Query<(Entity, &ChildOf, &GlobalTransform, &Collider), Without<PlayerMarker>>,
    wall_properties_query: &Query<&WallProperties>,
) -> Option<(WallProperties, f32, String)> {
    let max_interaction_distance = 100.0;
    let mut nearest_distance = f32::INFINITY;
    let mut nearest_wall = None;

    println!("find_nearest_by_distance_separate_queries");

    for (_entity, child_of, wall_transform, collider) in collider_query.iter() {
        // Check if this entity has wall properties
        if let Ok(wall_properties) = wall_properties_query.get(child_of.parent()) {
            println!("Found wall entity with properties");
            let distance = calculate_distance_to_collider(*player_pos, wall_transform, collider);

            if distance < max_interaction_distance && distance < nearest_distance {
                nearest_distance = distance;
                nearest_wall = Some((
                    wall_properties.clone(),
                    distance,
                    "Surface Distance".to_string(),
                ));
            }
        }
    }

    nearest_wall
}

pub fn calculate_distance_to_collider(
    point: Vec2,
    collider_transform: &GlobalTransform,
    collider: &Collider,
) -> f32 {
    let collider_pos = collider_transform.translation().truncate();
    let relative_point = point - collider_pos;

    if let Some(rect) = collider.shape().as_cuboid() {
        let half_width = rect.half_extents.x;
        let half_height = rect.half_extents.y;

        // Apply rotation if any
        let rotated_point = if collider_transform.rotation() != Quat::IDENTITY {
            let angle = collider_transform.rotation().to_euler(EulerRot::ZYX).0;
            let cos_a = angle.cos();
            let sin_a = angle.sin();
            Vec2::new(
                relative_point.x * cos_a + relative_point.y * sin_a,
                -relative_point.x * sin_a + relative_point.y * cos_a,
            )
        } else {
            relative_point
        };

        // Calculate distance to rectangle
        let dx = (rotated_point.x.abs() - half_width).max(0.0);
        let dy = (rotated_point.y.abs() - half_height).max(0.0);

        if rotated_point.x.abs() <= half_width && rotated_point.y.abs() <= half_height {
            return 0.0; // Or use negative distance: -dist_to_edge_x.min(dist_to_edge_y)
        } else {
            // Point is outside the rectangle
            return (dx * dx + dy * dy).sqrt();
        }
    }

    if let Some(poly) = collider.shape().as_polyline() {
        // For complex polygons, we'll use a simplified approach
        // In a real implementation, you might want to use proper polygon distance algorithms
        let vertices = poly.vertices();
        if vertices.is_empty() {
            return relative_point.length();
        }

        // Find min/max bounds
        let mut min_x = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for vertex in vertices {
            min_x = min_x.min(vertex.x);
            max_x = max_x.max(vertex.x);
            min_y = min_y.min(vertex.y);
            max_y = max_y.max(vertex.y);
        }

        let half_width = (max_x - min_x) / 2.0;
        let half_height = (max_y - min_y) / 2.0;

        // Use bounding box distance as approximation
        let dx = (relative_point.x.abs() - half_width).max(0.0);
        let dy = (relative_point.y.abs() - half_height).max(0.0);
        return (dx * dx + dy * dy).sqrt();
    }

    relative_point.length()
}
