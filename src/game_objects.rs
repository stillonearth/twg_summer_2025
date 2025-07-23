use bevy::prelude::*;
use std::fmt::Debug;

use crate::navigation::{GridPos, NavigateToTile, NavigationGrid, TileSize};
use crate::player::PlayerMarker;

pub struct GameObjectsPlugin;

// Add this event for object interaction navigation
#[derive(Event)]
pub struct NavigateToObjectEvent {
    pub object_position: Vec2,
    pub object_name: String,
}

// Marker component to indicate an object can be clicked for navigation
#[derive(Component)]
pub struct ClickNavigable;

// Update your existing GameObjectsPlugin
impl Plugin for GameObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<WallProperties>()
            .register_type::<WallType>()
            .register_type::<Bed>()
            .register_type::<Bath>()
            .register_type::<Kitchen>()
            .register_type::<Mirror>()
            .register_type::<ComputerDesk>()
            .register_type::<Couch>()
            .register_type::<WaterBottle>()
            .register_type::<Toilet>()
            .register_type::<Sink>()
            .register_type::<WalkableTile>()
            // Add the object navigation event
            .add_event::<NavigateToObjectEvent>()
            .add_systems(
                Update,
                (
                    // Existing hoverable systems
                    setup_bed_hoverable,
                    setup_bath_hoverable,
                    setup_kitchen_hoverable,
                    setup_toilet_hoverable,
                    setup_sink_hoverable,
                    setup_mirror_hoverable,
                    setup_computer_desk_hoverable,
                    setup_couch_hoverable,
                    setup_water_bottle_hoverable,
                    cleanup_orphaned_tooltips,
                    // New clickable systems
                    setup_bed_clickable,
                    setup_bath_clickable,
                    setup_kitchen_clickable,
                    setup_toilet_clickable,
                    setup_sink_clickable,
                    setup_mirror_clickable,
                    setup_computer_desk_clickable,
                    setup_couch_clickable,
                    setup_water_bottle_clickable,
                    // Object navigation handler
                    handle_object_navigation_events,
                ),
            );
    }
}

#[derive(Default, Reflect, Clone)]
#[reflect(Default)]
pub enum WallType {
    #[default]
    Stone,
}

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component, Default)]
pub struct WallProperties {
    pub name: String,
    pub wall_type: WallType,
}

// Trait for components that have a name field and can be made hoverable
pub trait NamedComponent: Component + Clone {
    fn name(&self) -> &str;
}

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component, Default)]
pub struct Bed {
    pub name: String,
}

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component, Default)]
pub struct WalkableTile {
    pub name: String,
}

impl NamedComponent for Bed {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component, Default)]
pub struct Bath {
    pub name: String,
}

impl NamedComponent for Bath {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component, Default)]
pub struct Kitchen {
    pub name: String,
}

impl NamedComponent for Kitchen {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component, Default)]
pub struct Mirror {
    pub name: String,
}

impl NamedComponent for Mirror {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component, Default)]
pub struct ComputerDesk {
    pub name: String,
}

impl NamedComponent for ComputerDesk {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component, Default)]
pub struct Couch {
    pub name: String,
}

impl NamedComponent for Couch {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component, Default)]
pub struct WaterBottle {
    pub name: String,
}

impl NamedComponent for WaterBottle {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component, Default)]
pub struct Toilet {
    pub name: String,
}

impl NamedComponent for Toilet {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component, Default)]
pub struct Sink {
    pub name: String,
}

impl NamedComponent for Sink {
    fn name(&self) -> &str {
        &self.name
    }
}

// Generic hoverable setup system
pub fn setup_hoverable<T: NamedComponent>(
    mut commands: Commands,
    query: Query<Entity, With<T>>,
    mut has_run: Local<bool>,
) {
    if *has_run {
        return;
    }

    for entity in query.iter() {
        commands
            .entity(entity)
            .insert((Pickable::default(), ZIndex(100)))
            .observe(recolor_same_component_on::<T, Pointer<Over>>(Color::srgb(
                0.0, 1.0, 1.0,
            )))
            .observe(recolor_same_component_on::<T, Pointer<Out>>(Color::srgba(
                1.0, 1.0, 1.0, 1.0,
            )))
            .observe(show_tooltip_on_hover::<T>)
            .observe(hide_tooltip_on_unhover::<T>);
    }

    let count = query.iter().count();

    if count != 0 {
        *has_run = true;
    }
}

// Generic recolor function
pub fn recolor_same_component_on<T: NamedComponent, E: Debug + Clone + Reflect>(
    color: Color,
) -> impl Fn(Trigger<E>, Query<&T>, Query<(Entity, &T)>, Query<&mut Sprite>) {
    move |ev, target_query, component_query, mut sprites| {
        let Ok(target_component) = target_query.get(ev.target()) else {
            return;
        };

        for (entity, component) in component_query.iter() {
            if component.name() == target_component.name() {
                if let Ok(mut sprite) = sprites.get_mut(entity) {
                    sprite.color = color;
                }
            }
        }
    }
}

// Specific system functions
pub fn setup_bed_hoverable(
    commands: Commands,
    query: Query<Entity, With<Bed>>,
    has_run: Local<bool>,
) {
    setup_hoverable::<Bed>(commands, query, has_run);
}

pub fn setup_bath_hoverable(
    commands: Commands,
    query: Query<Entity, With<Bath>>,
    has_run: Local<bool>,
) {
    setup_hoverable::<Bath>(commands, query, has_run);
}

pub fn setup_kitchen_hoverable(
    commands: Commands,
    query: Query<Entity, With<Kitchen>>,
    has_run: Local<bool>,
) {
    setup_hoverable::<Kitchen>(commands, query, has_run);
}

pub fn setup_mirror_hoverable(
    commands: Commands,
    query: Query<Entity, With<Mirror>>,
    has_run: Local<bool>,
) {
    setup_hoverable::<Mirror>(commands, query, has_run);
}

pub fn setup_computer_desk_hoverable(
    commands: Commands,
    query: Query<Entity, With<ComputerDesk>>,
    has_run: Local<bool>,
) {
    setup_hoverable::<ComputerDesk>(commands, query, has_run);
}

pub fn setup_couch_hoverable(
    commands: Commands,
    query: Query<Entity, With<Couch>>,
    has_run: Local<bool>,
) {
    setup_hoverable::<Couch>(commands, query, has_run);
}

pub fn setup_water_bottle_hoverable(
    commands: Commands,
    query: Query<Entity, With<WaterBottle>>,
    has_run: Local<bool>,
) {
    setup_hoverable::<WaterBottle>(commands, query, has_run);
}

pub fn setup_toilet_hoverable(
    commands: Commands,
    query: Query<Entity, With<Toilet>>,
    has_run: Local<bool>,
) {
    setup_hoverable::<Toilet>(commands, query, has_run);
}

pub fn setup_sink_hoverable(
    commands: Commands,
    query: Query<Entity, With<Sink>>,
    has_run: Local<bool>,
) {
    setup_hoverable::<Sink>(commands, query, has_run);
}

#[derive(Component)]
pub struct GameObjectTooltip {
    pub target_name: String,
}

pub fn show_tooltip_on_hover<T: NamedComponent>(
    trigger: Trigger<Pointer<Over>>,
    mut commands: Commands,
    target_query: Query<&T>,
    component_query: Query<(Entity, &T, &Transform)>,
    existing_tooltips: Query<Entity, With<GameObjectTooltip>>,
) {
    // Remove any existing tooltips
    for entity in existing_tooltips.iter() {
        commands.entity(entity).despawn();
    }

    // Get the name of the target component
    let Ok(target_component) = target_query.get(trigger.target()) else {
        return;
    };

    // Find all components with the same name and calculate center position
    let same_name_components: Vec<_> = component_query
        .iter()
        .filter(|(_, component, _)| component.name() == target_component.name())
        .collect();

    if same_name_components.is_empty() {
        return;
    }

    // Calculate center position of all components with same name
    let center = same_name_components
        .iter()
        .map(|(_, _, transform)| transform.translation)
        .fold(Vec3::ZERO, |acc, pos| acc + pos)
        / same_name_components.len() as f32;

    // Spawn tooltip entity at center position (slightly above)
    commands.spawn((
        GameObjectTooltip {
            target_name: target_component.name().to_string(),
        },
        Text::new(target_component.name().to_string()),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_translation(center + Vec3::new(0.0, 30.0, 1000.0)),
        ZIndex(1000),
    ));
}

pub fn hide_tooltip_on_unhover<T: NamedComponent>(
    _trigger: Trigger<Pointer<Out>>,
    mut commands: Commands,
    existing_tooltips: Query<Entity, With<GameObjectTooltip>>,
) {
    // Remove the tooltip
    for entity in existing_tooltips.iter() {
        if let Ok(mut ec) = commands.get_entity(entity) {
            ec.despawn();
        }
    }
}

// Cleanup system to remove orphaned tooltips
pub fn cleanup_orphaned_tooltips(
    mut commands: Commands,
    tooltips: Query<(Entity, &GameObjectTooltip)>,
    // Check if any component with the tooltip's target name still exists
    beds: Query<&Bed>,
    baths: Query<&Bath>,
    kitchens: Query<&Kitchen>,
    mirrors: Query<&Mirror>,
    computer_desks: Query<&ComputerDesk>,
    couches: Query<&Couch>,
    water_bottles: Query<&WaterBottle>,
    toilets: Query<&Toilet>,
    sinks: Query<&Sink>,
) {
    for (tooltip_entity, tooltip) in tooltips.iter() {
        let name_exists = beds.iter().any(|bed| bed.name == tooltip.target_name)
            || baths.iter().any(|bath| bath.name == tooltip.target_name)
            || kitchens
                .iter()
                .any(|kitchen| kitchen.name == tooltip.target_name)
            || mirrors
                .iter()
                .any(|mirror| mirror.name == tooltip.target_name)
            || computer_desks
                .iter()
                .any(|desk| desk.name == tooltip.target_name)
            || couches
                .iter()
                .any(|couch| couch.name == tooltip.target_name)
            || water_bottles
                .iter()
                .any(|bottle| bottle.name == tooltip.target_name)
            || toilets
                .iter()
                .any(|toilet| toilet.name == tooltip.target_name)
            || sinks.iter().any(|sink| sink.name == tooltip.target_name);

        if !name_exists {
            if let Ok(mut ec) = commands.get_entity(tooltip_entity) {
                ec.despawn();
            }
        }
    }
}

// Generic function to add click navigation to any NamedComponent
pub fn setup_object_clickable<T: NamedComponent>(
    mut commands: Commands,
    query: Query<Entity, (With<T>, Without<ClickNavigable>)>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(ClickNavigable)
            .observe(navigate_to_object_on_click::<T>);
    }
}

// Generic observer function for handling clicks on objects
pub fn navigate_to_object_on_click<T: NamedComponent>(
    trigger: Trigger<Pointer<Click>>,
    mut navigation_events: EventWriter<NavigateToObjectEvent>,
    target_query: Query<(&T, &Transform)>,
) {
    let Ok((target_component, target_transform)) = target_query.get(trigger.target()) else {
        return;
    };

    // Send navigation event to move to this object
    navigation_events.send(NavigateToObjectEvent {
        object_position: target_transform.translation.truncate(),
        object_name: target_component.name().to_string(),
    });

    info!("Clicked on {}, navigating player", target_component.name());
}

// System to handle object navigation events and find nearest walkable tile
pub fn handle_object_navigation_events(
    mut object_nav_events: EventReader<NavigateToObjectEvent>,
    mut tile_nav_events: EventWriter<NavigateToTile>,
    player_query: Query<&Transform, With<PlayerMarker>>,
    walkable_tiles_query: Query<&Transform, (With<WalkableTile>, Without<PlayerMarker>)>,
    navigation_grid: Res<NavigationGrid>,
    tile_size: Res<TileSize>,
) {
    for event in object_nav_events.read() {
        let Ok(player_transform) = player_query.get_single() else {
            continue;
        };

        let player_grid_pos =
            navigation_grid.world_to_grid(player_transform.translation, tile_size.0);
        let object_position = event.object_position;

        // Find the nearest walkable tile to the object
        let nearest_walkable_grid_pos = find_nearest_walkable_tile(
            object_position,
            &walkable_tiles_query,
            &navigation_grid,
            &tile_size,
        );

        if let Some(target_grid_pos) = nearest_walkable_grid_pos {
            // Send tile navigation event to trigger pathfinding
            tile_nav_events.send(NavigateToTile {
                from: player_grid_pos,
                to: target_grid_pos,
            });

            info!(
                "Moving from {:?} to nearest walkable tile {:?} for object: {}",
                player_grid_pos, target_grid_pos, event.object_name
            );
        } else {
            warn!("No walkable tiles found near object: {}", event.object_name);
        }
    }
}

// Helper function to find the nearest walkable tile to a given position
fn find_nearest_walkable_tile(
    target_position: Vec2,
    walkable_tiles_query: &Query<&Transform, (With<WalkableTile>, Without<PlayerMarker>)>,
    navigation_grid: &NavigationGrid,
    tile_size: &TileSize,
) -> Option<GridPos> {
    let mut nearest_tile: Option<(GridPos, f32)> = None;

    for tile_transform in walkable_tiles_query.iter() {
        let tile_position = tile_transform.translation.truncate();
        let distance = target_position.distance(tile_position);
        let grid_pos = navigation_grid.world_to_grid(tile_transform.translation, tile_size.0);

        // Make sure the tile is actually walkable in our grid
        if !navigation_grid.is_walkable(grid_pos) {
            continue;
        }

        match nearest_tile {
            None => {
                nearest_tile = Some((grid_pos, distance));
            }
            Some((_, current_distance)) if distance < current_distance => {
                nearest_tile = Some((grid_pos, distance));
            }
            _ => {}
        }
    }

    nearest_tile.map(|(grid_pos, _)| grid_pos)
}

// Specific setup systems for each object type
pub fn setup_bed_clickable(
    commands: Commands,
    query: Query<Entity, (With<Bed>, Without<ClickNavigable>)>,
) {
    setup_object_clickable::<Bed>(commands, query);
}

pub fn setup_bath_clickable(
    commands: Commands,
    query: Query<Entity, (With<Bath>, Without<ClickNavigable>)>,
) {
    setup_object_clickable::<Bath>(commands, query);
}

pub fn setup_kitchen_clickable(
    commands: Commands,
    query: Query<Entity, (With<Kitchen>, Without<ClickNavigable>)>,
) {
    setup_object_clickable::<Kitchen>(commands, query);
}

pub fn setup_mirror_clickable(
    commands: Commands,
    query: Query<Entity, (With<Mirror>, Without<ClickNavigable>)>,
) {
    setup_object_clickable::<Mirror>(commands, query);
}

pub fn setup_computer_desk_clickable(
    commands: Commands,
    query: Query<Entity, (With<ComputerDesk>, Without<ClickNavigable>)>,
) {
    setup_object_clickable::<ComputerDesk>(commands, query);
}

pub fn setup_couch_clickable(
    commands: Commands,
    query: Query<Entity, (With<Couch>, Without<ClickNavigable>)>,
) {
    setup_object_clickable::<Couch>(commands, query);
}

pub fn setup_water_bottle_clickable(
    commands: Commands,
    query: Query<Entity, (With<WaterBottle>, Without<ClickNavigable>)>,
) {
    setup_object_clickable::<WaterBottle>(commands, query);
}

pub fn setup_toilet_clickable(
    commands: Commands,
    query: Query<Entity, (With<Toilet>, Without<ClickNavigable>)>,
) {
    setup_object_clickable::<Toilet>(commands, query);
}

pub fn setup_sink_clickable(
    commands: Commands,
    query: Query<Entity, (With<Sink>, Without<ClickNavigable>)>,
) {
    setup_object_clickable::<Sink>(commands, query);
}
