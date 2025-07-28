use bevy::prelude::*;
use std::fmt::Debug;

use crate::navigation::{GridPos, NavigateToTile, NavigationGrid, TileSize};
use crate::player::PlayerMarker;

pub struct GameObjectsPlugin;

// Events
#[derive(Event)]
pub struct NavigateToObjectEvent {
    pub object_name: String,
}

// Components
#[derive(Component)]
pub struct ClickNavigable;

#[derive(Component)]
pub struct GameObjectTooltip {
    pub target_name: String,
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

// Macro to generate game object components
macro_rules! define_game_object {
    ($name:ident) => {
        #[derive(Component, Reflect, Default, Clone)]
        #[reflect(Component, Default)]
        pub struct $name {
            pub name: String,
        }

        impl NamedComponent for $name {
            fn name(&self) -> &str {
                &self.name
            }
        }
    };
}

// Define all game objects using the macro
define_game_object!(Bed);
define_game_object!(Bath);
define_game_object!(Kitchen);
define_game_object!(Mirror);
define_game_object!(ComputerDesk);
define_game_object!(Couch);
define_game_object!(WaterBottle);
define_game_object!(Toilet);
define_game_object!(Sink);
define_game_object!(GameConsole);
define_game_object!(Phone);
define_game_object!(WalkableTile);

// Registry of all game object types for automation
pub struct GameObjectRegistry;

impl GameObjectRegistry {
    pub fn register_types(app: &mut App) {
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
            .register_type::<GameConsole>()
            .register_type::<Phone>()
            .register_type::<WalkableTile>();
    }
}

impl Plugin for GameObjectsPlugin {
    fn build(&self, app: &mut App) {
        GameObjectRegistry::register_types(app);

        app.add_event::<NavigateToObjectEvent>().add_systems(
            Update,
            (cleanup_orphaned_tooltips, handle_object_navigation_events),
        );
    }
}

// Generic setup functions
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

// Generic observer functions
pub fn recolor_same_component_on<T: NamedComponent, E: Debug + Clone + Reflect>(
    color: Color,
) -> impl Fn(Trigger<E>, Query<&T>, Query<(Entity, &T)>, Query<&mut Sprite>) {
    move |ev, target_query, component_query, mut sprites| {
        let Ok(target_component) = target_query.get(ev.target()) else {
            return;
        };

        for (entity, component) in component_query.iter() {
            if component.name() == target_component.name()
                && let Ok(mut sprite) = sprites.get_mut(entity)
            {
                sprite.color = color;
            }
        }
    }
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
    let Ok(target_component) = target_query.get(trigger.target()) else {
        return;
    };
    // Find all components with the same name
    let same_name_components: Vec<_> = component_query
        .iter()
        .filter(|(_, component, _)| component.name() == target_component.name())
        .collect();
    if same_name_components.is_empty() {
        return;
    }
    // Find the highest Y position (maximum Y value)
    let highest_y = same_name_components
        .iter()
        .map(|(_, _, transform)| transform.translation.y)
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap_or(0.0);
    // Find the leftmost X position (minimum X value)
    let leftmost_x = same_name_components
        .iter()
        .map(|(_, _, transform)| transform.translation.x)
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap_or(0.0);
    // Position tooltip at the left edge above the highest row
    let tooltip_position = Vec3::new(
        leftmost_x + 64.0 - 16.0, // Left X of all components
        highest_y + 32.0,         // Above the highest component
        100.0,                    // Z position for visibility
    );
    // Spawn tooltip as a world space text entity
    commands.spawn((
        GameObjectTooltip {
            target_name: target_component.name().to_string(),
        },
        Text2d::new(target_component.name().to_string()),
        TextFont {
            font_size: 15.0,
            ..default()
        },
        TextColor(Color::BLACK),
        Transform::from_translation(tooltip_position),
        // Add a background for better visibility
        Sprite {
            color: Color::srgba(1.0, 1.0, 1.0, 0.8), // Fixed alpha value (was 1.8 which is invalid)
            custom_size: Some(Vec2::new(
                target_component.name().len() as f32 * 12.0 + 10.0,
                25.0,
            )),
            ..default()
        },
        ZIndex(5),
    ));
}

pub fn hide_tooltip_on_unhover<T: NamedComponent>(
    _trigger: Trigger<Pointer<Out>>,
    mut commands: Commands,
    existing_tooltips: Query<Entity, With<GameObjectTooltip>>,
) {
    for entity in existing_tooltips.iter() {
        if let Ok(mut ec) = commands.get_entity(entity) {
            ec.despawn();
        }
    }
}

// Navigation and cleanup systems
pub fn handle_object_navigation_events(
    mut object_nav_events: EventReader<NavigateToObjectEvent>,
    mut tile_nav_events: EventWriter<NavigateToTile>,
    player_query: Query<&Transform, With<PlayerMarker>>,
    walkable_tiles_query: Query<&Transform, (With<WalkableTile>, Without<PlayerMarker>)>,
    navigation_grid: Res<NavigationGrid>,
    tile_size: Res<TileSize>,
    mut q_objects: ParamSet<(
        Query<(Entity, &Transform, &Bed)>,
        Query<(Entity, &Transform, &Kitchen)>,
        Query<(Entity, &Transform, &ComputerDesk)>,
        Query<(Entity, &Transform, &Sink)>,
        Query<(Entity, &Transform, &GameConsole)>,
        Query<(Entity, &Transform, &Phone)>,
    )>,
) {
    for event in object_nav_events.read() {
        let Ok(player_transform) = player_query.single() else {
            continue;
        };

        let player_grid_pos =
            navigation_grid.world_to_grid(player_transform.translation, tile_size.0);

        if let Some(object_position) = match event.object_name.as_ref() {
            "bed" => {
                let beds = q_objects.p0();
                let transforms: Vec<Vec3> = beds
                    .iter()
                    .map(|(_, transform, _)| transform.translation)
                    .collect();
                let mean_translation = transforms.iter().sum::<Vec3>() / transforms.len() as f32;
                Some(mean_translation)
            }
            "kitchen" => {
                let kitchens = q_objects.p1();
                let transforms: Vec<Vec3> = kitchens
                    .iter()
                    .map(|(_, transform, _)| transform.translation)
                    .collect();
                let mean_translation = transforms.iter().sum::<Vec3>() / transforms.len() as f32;
                Some(mean_translation)
            }
            "computer" => {
                let computerdesks = q_objects.p2();
                let transforms: Vec<Vec3> = computerdesks
                    .iter()
                    .map(|(_, transform, _)| transform.translation)
                    .collect();
                let mean_translation = transforms.iter().sum::<Vec3>() / transforms.len() as f32;
                Some(mean_translation)
            }
            "fridge" => {
                let kitchens = q_objects.p1();
                let transforms: Vec<Vec3> = kitchens
                    .iter()
                    .map(|(_, transform, _)| transform.translation)
                    .collect();
                let mean_translation = transforms.iter().sum::<Vec3>() / transforms.len() as f32;
                Some(mean_translation)
            }
            "phone" => {
                let phones = q_objects.p5();
                let transforms: Vec<Vec3> = phones
                    .iter()
                    .map(|(_, transform, _)| transform.translation)
                    .collect();
                let mean_translation = transforms.iter().sum::<Vec3>() / transforms.len() as f32;
                Some(mean_translation)
            }
            "bathroom" => {
                let sinks = q_objects.p3();
                let transforms: Vec<Vec3> = sinks
                    .iter()
                    .map(|(_, transform, _)| transform.translation)
                    .collect();
                let mean_translation = transforms.iter().sum::<Vec3>() / transforms.len() as f32;
                Some(mean_translation)
            }
            "tv" => {
                let gameconsoles = q_objects.p4();
                let transforms: Vec<Vec3> = gameconsoles
                    .iter()
                    .map(|(_, transform, _)| transform.translation)
                    .collect();
                let mean_translation = transforms.iter().sum::<Vec3>() / transforms.len() as f32;
                Some(mean_translation)
            }
            "gameconsole" => {
                let gameconsoles = q_objects.p4();
                let transforms: Vec<Vec3> = gameconsoles
                    .iter()
                    .map(|(_, transform, _)| transform.translation)
                    .collect();
                let mean_translation = transforms.iter().sum::<Vec3>() / transforms.len() as f32;
                Some(mean_translation)
            }
            _ => None,
        } {
            if let Some(target_grid_pos) = find_nearest_walkable_tile(
                object_position.truncate(),
                &walkable_tiles_query,
                &navigation_grid,
                &tile_size,
            ) {
                tile_nav_events.write(NavigateToTile {
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
}

fn find_nearest_walkable_tile(
    target_position: Vec2,
    walkable_tiles_query: &Query<&Transform, (With<WalkableTile>, Without<PlayerMarker>)>,
    navigation_grid: &NavigationGrid,
    tile_size: &TileSize,
) -> Option<GridPos> {
    walkable_tiles_query
        .iter()
        .filter_map(|tile_transform| {
            let tile_position = tile_transform.translation.truncate();
            let grid_pos = navigation_grid.world_to_grid(tile_transform.translation, tile_size.0);

            if navigation_grid.is_walkable(grid_pos) {
                Some((grid_pos, target_position.distance(tile_position)))
            } else {
                None
            }
        })
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(grid_pos, _)| grid_pos)
}

// Simplified cleanup system using direct queries
pub fn cleanup_orphaned_tooltips(
    mut commands: Commands,
    tooltips: Query<(Entity, &GameObjectTooltip)>,
    // Direct queries for each component type
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
        let name_exists = beds.iter().any(|comp| comp.name() == tooltip.target_name)
            || baths.iter().any(|comp| comp.name() == tooltip.target_name)
            || kitchens
                .iter()
                .any(|comp| comp.name() == tooltip.target_name)
            || mirrors
                .iter()
                .any(|comp| comp.name() == tooltip.target_name)
            || computer_desks
                .iter()
                .any(|comp| comp.name() == tooltip.target_name)
            || couches
                .iter()
                .any(|comp| comp.name() == tooltip.target_name)
            || water_bottles
                .iter()
                .any(|comp| comp.name() == tooltip.target_name)
            || toilets
                .iter()
                .any(|comp| comp.name() == tooltip.target_name)
            || sinks.iter().any(|comp| comp.name() == tooltip.target_name);

        if !name_exists && let Ok(mut ec) = commands.get_entity(tooltip_entity) {
            ec.despawn();
        }
    }
}
