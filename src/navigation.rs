use bevy::prelude::*;
use rand::Rng;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::{game_objects::WalkableTile, player::PlayerMarker};

const TILE_SIZE: f32 = 32.0;

pub struct NavigationGridPlugin;

#[derive(Event, Debug)]
pub struct GoToRandomTile;

impl Plugin for NavigationGridPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TileSize(TILE_SIZE))
            .insert_resource(NavigationGrid::new())
            .add_systems(
                Update,
                (
                    (setup_walkable_tile_handlers, setup_navigation_grid).chain(),
                    handle_navigation_event,
                    handle_go_to_random_tile,
                ),
            )
            .add_event::<NavigateToTile>()
            .add_event::<MovePlayerCommand>()
            .add_event::<GoToRandomTile>();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}

#[derive(Event, Debug)]
pub struct NavigateToTile {
    pub from: GridPos,
    pub to: GridPos,
}

#[derive(Event, Debug)]
pub struct MovePlayerCommand {
    pub path: Vec<GridPos>,
}

// Node for A* pathfinding
#[derive(Debug, Clone, PartialEq, Eq)]
struct AStarNode {
    pos: GridPos,
    f_cost: i32,
    g_cost: i32,
    h_cost: i32,
}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_cost.cmp(&self.f_cost) // Reverse for min-heap behavior
    }
}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Resource)]
pub struct TileSize(pub f32);

#[derive(Resource)]
pub struct NavigationGrid {
    pub walkable: HashSet<(i32, i32)>,
}

impl NavigationGrid {
    pub fn new() -> Self {
        Self {
            walkable: HashSet::new(),
        }
    }

    pub fn get_random_walkable_tile(&self) -> Option<GridPos> {
        if self.walkable.is_empty() {
            return None;
        }

        let tiles: Vec<&(i32, i32)> = self.walkable.iter().collect();
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..tiles.len());

        if let Some(&(x, y)) = tiles.get(random_index) {
            Some(GridPos { x: *x, y: *y })
        } else {
            None
        }
    }

    /// Get a random walkable tile that's different from the current position
    pub fn get_random_walkable_tile_excluding(&self, exclude: GridPos) -> Option<GridPos> {
        if self.walkable.len() <= 1 {
            return None;
        }

        let tiles: Vec<&(i32, i32)> = self
            .walkable
            .iter()
            .filter(|&&(x, y)| GridPos { x, y } != exclude)
            .collect();

        if tiles.is_empty() {
            return None;
        }

        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..tiles.len());

        if let Some(&(x, y)) = tiles.get(random_index) {
            Some(GridPos { x: *x, y: *y })
        } else {
            None
        }
    }

    pub fn add_walkable_tile(&mut self, x: i32, y: i32) {
        self.walkable.insert((x, y));
    }

    pub fn world_to_grid(&self, world_pos: Vec3, tile_size: f32) -> GridPos {
        GridPos {
            x: (world_pos.x / tile_size).floor() as i32,
            y: (world_pos.y / tile_size).floor() as i32,
        }
    }

    pub fn is_walkable(&self, pos: GridPos) -> bool {
        self.walkable.contains(&(pos.x, pos.y))
    }

    pub fn find_path(&self, start: GridPos, goal: GridPos) -> Option<Vec<GridPos>> {
        if !self.is_walkable(start) || !self.is_walkable(goal) {
            return None;
        }

        if start == goal {
            return Some(vec![start]);
        }

        let mut open_set = BinaryHeap::new();
        let mut came_from: HashMap<GridPos, GridPos> = HashMap::new();
        let mut g_score: HashMap<GridPos, i32> = HashMap::new();

        g_score.insert(start, 0);
        open_set.push(AStarNode {
            pos: start,
            g_cost: 0,
            h_cost: Self::heuristic(start, goal),
            f_cost: Self::heuristic(start, goal),
        });

        while let Some(current_node) = open_set.pop() {
            let current_pos = current_node.pos;

            if current_pos == goal {
                return Some(Self::reconstruct_path(&came_from, current_pos));
            }

            for neighbor in Self::get_neighbors(current_pos) {
                if !self.is_walkable(neighbor) {
                    continue;
                }

                let tentative_g_score = g_score.get(&current_pos).unwrap_or(&i32::MAX) + 1;

                if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&i32::MAX) {
                    came_from.insert(neighbor, current_pos);
                    g_score.insert(neighbor, tentative_g_score);

                    let h_cost = Self::heuristic(neighbor, goal);
                    open_set.push(AStarNode {
                        pos: neighbor,
                        g_cost: tentative_g_score,
                        h_cost,
                        f_cost: tentative_g_score + h_cost,
                    });
                }
            }
        }

        None // No path found
    }

    fn heuristic(a: GridPos, b: GridPos) -> i32 {
        (a.x - b.x).abs() + (a.y - b.y).abs() // Manhattan distance
    }

    fn get_neighbors(pos: GridPos) -> Vec<GridPos> {
        vec![
            GridPos {
                x: pos.x + 1,
                y: pos.y,
            }, // Right
            GridPos {
                x: pos.x - 1,
                y: pos.y,
            }, // Left
            GridPos {
                x: pos.x,
                y: pos.y + 1,
            }, // Up
            GridPos {
                x: pos.x,
                y: pos.y - 1,
            }, // Down
        ]
    }

    fn reconstruct_path(
        came_from: &HashMap<GridPos, GridPos>,
        mut current: GridPos,
    ) -> Vec<GridPos> {
        let mut path = vec![current];

        while let Some(&parent) = came_from.get(&current) {
            current = parent;
            path.push(current);
        }

        path.reverse();
        path
    }
}

fn handle_go_to_random_tile(
    mut random_events: EventReader<GoToRandomTile>,
    mut nav_events: EventWriter<NavigateToTile>,
    player_query: Query<&Transform, With<PlayerMarker>>,
    navigation_grid: Res<NavigationGrid>,
    tile_size: Res<TileSize>,
) {
    for _event in random_events.read() {
        let Ok(player_transform) = player_query.single() else {
            println!("No player found for random navigation");
            continue;
        };

        let player_grid_pos =
            navigation_grid.world_to_grid(player_transform.translation, tile_size.0);

        // Get a random walkable tile that's different from current position
        if let Some(random_tile) =
            navigation_grid.get_random_walkable_tile_excluding(player_grid_pos)
        {
            println!("Moving player to random tile: {random_tile:?}");

            nav_events.write(NavigateToTile {
                from: player_grid_pos,
                to: random_tile,
            });
        } else {
            println!("No valid random tiles available for navigation");
        }
    }
}

fn setup_navigation_grid(
    mut nav_grid: ResMut<NavigationGrid>,
    walkable_tiles: Query<&Transform, With<WalkableTile>>,
    tile_size: Res<TileSize>,
    mut initialized: Local<bool>,
) {
    // Only run once when we have walkable tiles
    if *initialized || walkable_tiles.is_empty() {
        return;
    }

    for transform in walkable_tiles.iter() {
        let world_pos = transform.translation.truncate();
        let grid_pos = (
            (world_pos.x / tile_size.0).floor() as i32,
            (world_pos.y / tile_size.0).floor() as i32,
        );
        nav_grid.add_walkable_tile(grid_pos.0, grid_pos.1);
    }

    println!(
        "Navigation grid initialized with {} walkable tiles",
        nav_grid.walkable.len()
    );
    *initialized = true;
}

fn handle_tile_click(
    trigger: Trigger<Pointer<Click>>,
    mut nav_events: EventWriter<NavigateToTile>,
    player_query: Query<&Transform, (With<PlayerMarker>, Without<WalkableTile>)>,
    tile_query: Query<&Transform, (With<WalkableTile>, Without<PlayerMarker>)>,
    navigation_grid: Res<NavigationGrid>,
    tile_size: Res<TileSize>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let Ok(tile_transform) = tile_query.get(trigger.target) else {
        return;
    };

    let player_grid_pos = navigation_grid.world_to_grid(player_transform.translation, tile_size.0);
    let target_grid_pos = navigation_grid.world_to_grid(tile_transform.translation, tile_size.0);

    // Use EventWriter instead of commands.trigger
    nav_events.write(NavigateToTile {
        from: player_grid_pos,
        to: target_grid_pos,
    });
}

fn handle_navigation_event(
    mut nav_events: EventReader<NavigateToTile>,
    mut mov_cmds: EventWriter<MovePlayerCommand>,
    navigation_grid: Res<NavigationGrid>,
) {
    for event in nav_events.read() {
        println!("Pathfinding from {:?} to {:?}", event.from, event.to);

        if let Some(path) = navigation_grid.find_path(event.from, event.to) {
            println!("Path found with {} steps: {:?}", path.len(), path);

            mov_cmds.write(MovePlayerCommand { path });
        } else {
            println!("No path found!");
        }
    }
}

pub fn setup_walkable_tile_handlers(
    commands: Commands,
    query: Query<Entity, With<WalkableTile>>,
    mut sprite_query: Query<&mut Sprite>,
    mut has_run: Local<bool>,
) {
    if *has_run {
        return;
    }
    for entity in query.iter() {
        // commands
        // .entity(entity)
        // .insert(Pickable::default())
        // .observe(handle_tile_click);

        // Make sprite transparent
        if let Ok(mut sprite) = sprite_query.get_mut(entity) {
            sprite.color = Color::NONE; // or Color::rgba(0.0, 0.0, 0.0, 0.0)
        }
    }
    if !query.is_empty() {
        *has_run = true;
    }
}
