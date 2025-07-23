use bevy::prelude::*;
use std::collections::HashSet;

use crate::{game_objects::WalkableTile, player::PlayerMarker};

const TILE_SIZE: f32 = 32.0;

pub struct NavigationGridPlugin {
    pub tile_size: f32,
}

impl Default for NavigationGridPlugin {
    fn default() -> Self {
        Self {
            tile_size: TILE_SIZE,
        }
    }
}

impl NavigationGridPlugin {
    pub fn new(tile_size: f32) -> Self {
        Self { tile_size }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}

#[derive(Event, Debug)]
pub struct NavigateToTile {
    pub from: GridPos,
    pub to: GridPos,
    pub target_world_pos: Vec3,
}

impl Plugin for NavigationGridPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TileSize(self.tile_size))
            .add_systems(
                Update,
                (setup_navigation_grid, setup_walkable_tile_handlers),
            )
            .add_event::<NavigateToTile>();
    }
}

#[derive(Resource)]
pub struct TileSize(pub f32);

#[derive(Resource)]
pub struct NavigationGrid {
    pub width: u32,
    pub height: u32,
    pub walkable: HashSet<(i32, i32)>, // Set of walkable tile coordinates
    pub offset_x: i32,                 // Offset for handling negative coordinates
    pub offset_y: i32,
}

impl NavigationGrid {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            walkable: HashSet::new(),
            offset_x: 0,
            offset_y: 0,
        }
    }

    pub fn add_walkable_tile(&mut self, x: i32, y: i32) {
        self.walkable.insert((x, y));
    }

    pub fn is_walkable(&self, x: i32, y: i32) -> bool {
        self.walkable.contains(&(x, y))
    }

    pub fn world_to_grid(&self, world_pos: Vec3, tile_size: f32) -> GridPos {
        GridPos {
            x: (world_pos.x / tile_size).floor() as i32,
            y: (world_pos.y / tile_size).floor() as i32,
        }
    }

    pub fn grid_to_world(&self, grid_pos: (i32, i32), tile_size: f32) -> Vec2 {
        Vec2::new(
            grid_pos.0 as f32 * tile_size + tile_size / 2.0,
            grid_pos.1 as f32 * tile_size + tile_size / 2.0,
        )
    }

    pub fn get_walkable_tiles(&self) -> &HashSet<(i32, i32)> {
        &self.walkable
    }

    pub fn tile_count(&self) -> usize {
        self.walkable.len()
    }
}

// Function to initialize navigation grid from WalkableTile entities
fn setup_navigation_grid(
    mut commands: Commands,
    walkable_tiles: Query<&Transform, With<WalkableTile>>,
    tile_size: Res<TileSize>,
    existing_grid: Option<Res<NavigationGrid>>,
) {
    // Only create if we don't have a grid and we have walkable tiles
    if existing_grid.is_some() || walkable_tiles.is_empty() {
        return;
    }

    // Calculate map bounds from walkable tiles
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    let tile_positions: Vec<(i32, i32)> = walkable_tiles
        .iter()
        .map(|transform| {
            let world_pos = transform.translation.truncate();
            let grid_pos = (
                (world_pos.x / tile_size.0).floor() as i32,
                (world_pos.y / tile_size.0).floor() as i32,
            );

            min_x = min_x.min(grid_pos.0);
            max_x = max_x.max(grid_pos.0);
            min_y = min_y.min(grid_pos.1);
            max_y = max_y.max(grid_pos.1);

            grid_pos
        })
        .collect();

    let width = (max_x - min_x + 1) as u32;
    let height = (max_y - min_y + 1) as u32;

    let mut nav_grid = NavigationGrid::new(width, height);
    nav_grid.offset_x = min_x;
    nav_grid.offset_y = min_y;

    // Add all walkable tile positions
    for grid_pos in tile_positions {
        nav_grid.add_walkable_tile(grid_pos.0, grid_pos.1);
    }

    println!(
        "Navigation grid created: {}x{} tiles, {} walkable tiles",
        width,
        height,
        nav_grid.walkable.len()
    );

    commands.insert_resource(nav_grid);
}

pub fn setup_walkable_tile_handlers(
    mut commands: Commands,
    query: Query<Entity, With<WalkableTile>>,
    mut has_run: Local<bool>,
) {
    if *has_run {
        return;
    }

    for entity in query.iter() {
        commands
            .entity(entity)
            .insert((Pickable::default()))
            .observe(handle_tile_click);
    }

    let count = query.iter().count();

    if count != 0 {
        *has_run = true;
    }
}

fn handle_tile_click(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    player_query: Query<&Transform, (With<PlayerMarker>, Without<WalkableTile>)>,
    tile_query: Query<&Transform, (With<WalkableTile>, Without<PlayerMarker>)>,
    navigation_grid: Res<NavigationGrid>, // Your navigation grid resource
    tile_size: Res<TileSize>,
) {
    let clicked_entity = trigger.target;

    println!("clicked");

    // Get player position
    let Ok(player_transform) = player_query.get_single() else {
        warn!("No player found or multiple players exist");
        return;
    };

    // Get clicked tile position
    let Ok(tile_transform) = tile_query.get(clicked_entity) else {
        warn!("Clicked entity is not a valid walkable tile");
        return;
    };

    // Convert world positions to grid coordinates
    let player_grid_pos = navigation_grid.world_to_grid(player_transform.translation, tile_size.0);
    let target_grid_pos = navigation_grid.world_to_grid(tile_transform.translation, tile_size.0);

    let event = NavigateToTile {
        from: player_grid_pos,
        to: target_grid_pos,
        target_world_pos: tile_transform.translation,
    };

    println!("~~ {:?} ~~", event);

    // Trigger navigation event
    commands.trigger(event);
}
