use bevy::{prelude::*, text::FontStyle};
use std::fmt::Debug;

pub struct GameObjectsPlugin;

impl Plugin for GameObjectsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register all the component types for reflection
            .register_type::<WallProperties>()
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
            // Add all the hoverable setup systems
            .add_systems(
                Update,
                (
                    setup_bed_hoverable,
                    setup_bath_hoverable,
                    setup_kitchen_hoverable,
                    setup_toilet_hoverable,
                    setup_sink_hoverable,
                    setup_mirror_hoverable,
                    setup_computer_desk_hoverable,
                    setup_couch_hoverable,
                    setup_water_bottle_hoverable,
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

impl std::fmt::Display for WallType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WallType::Stone => write!(f, "Stone"),
        }
    }
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
            )));
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
