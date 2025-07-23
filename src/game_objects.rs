use bevy::{prelude::*, text::FontStyle};
use std::fmt::Debug;

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

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component, Default)]
pub struct Bed {
    pub name: String,
}

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component, Default)]
pub struct Bath {
    pub name: String,
}

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component, Default)]
pub struct Kitchen {
    pub name: String,
}

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component, Default)]
pub struct Mirror {
    pub name: String,
}

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component, Default)]
pub struct ComputerDesk {
    pub name: String,
}

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component, Default)]
pub struct Couch {
    pub name: String,
}

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component, Default)]
pub struct WaterBottle {
    pub name: String,
}

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component, Default)]
pub struct Toilet {
    pub name: String,
}

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component, Default)]
pub struct Sink {
    pub name: String,
}

pub fn setup_bed_hoverable(
    mut commands: Commands,
    bed_query: Query<Entity, With<Bed>>,
    mut has_run: Local<bool>,
) {
    if *has_run {
        return;
    }

    for entity in bed_query.iter() {
        commands
            .entity(entity)
            .insert(Pickable::default())
            .observe(recolor_same_bed_on::<Pointer<Over>>(Color::srgb(
                0.0, 1.0, 1.0,
            )))
            .observe(recolor_same_bed_on::<Pointer<Out>>(Color::srgba(
                1.0, 1.0, 1.0, 1.0,
            )));
    }

    let count = bed_query.iter().count();

    if count != 0 {
        *has_run = true;
    }
}

pub fn setup_bath_hoverable(
    mut commands: Commands,
    bath_query: Query<Entity, With<Bath>>,
    mut has_run: Local<bool>,
) {
    if *has_run {
        return;
    }

    for entity in bath_query.iter() {
        commands
            .entity(entity)
            .insert((Pickable::default(), ZIndex(100)))
            .observe(recolor_same_bath_on::<Pointer<Over>>(Color::srgb(
                0.0, 1.0, 1.0,
            )))
            .observe(recolor_same_bath_on::<Pointer<Out>>(Color::srgba(
                1.0, 1.0, 1.0, 1.0,
            )));
    }

    let count = bath_query.iter().count();

    if count != 0 {
        *has_run = true;
    }
}

pub fn recolor_same_bath_on<E: Debug + Clone + Reflect>(
    color: Color,
) -> impl Fn(Trigger<E>, Query<&Bath>, Query<(Entity, &Bath)>, Query<&mut Sprite>) {
    move |ev, target_bath_query, bath_query, mut sprites| {
        // Get the name of the target bath
        let Ok(target_bath) = target_bath_query.get(ev.target()) else {
            return;
        };

        // Find and recolor all baths with the same name
        for (entity, bath) in bath_query.iter() {
            if bath.name == target_bath.name {
                if let Ok(mut sprite) = sprites.get_mut(entity) {
                    sprite.color = color;
                }
            }
        }
    }
}

pub fn recolor_same_bed_on<E: Debug + Clone + Reflect>(
    color: Color,
) -> impl Fn(Trigger<E>, Query<&Bed>, Query<(Entity, &Bed)>, Query<&mut Sprite>) {
    move |ev, target_bed_query, bath_query, mut sprites| {
        // Get the name of the target bath
        let Ok(target_bed) = target_bed_query.get(ev.target()) else {
            return;
        };

        // Find and recolor all baths with the same name
        for (entity, bath) in bath_query.iter() {
            if bath.name == target_bed.name {
                if let Ok(mut sprite) = sprites.get_mut(entity) {
                    sprite.color = color;
                }
            }
        }
    }
}

// Kitchen implementations
pub fn setup_kitchen_hoverable(
    mut commands: Commands,
    kitchen_query: Query<Entity, With<Kitchen>>,
    mut has_run: Local<bool>,
) {
    if *has_run {
        return;
    }

    for entity in kitchen_query.iter() {
        commands
            .entity(entity)
            .insert((Pickable::default(), ZIndex(100)))
            .observe(recolor_same_kitchen_on::<Pointer<Over>>(Color::srgb(
                0.0, 1.0, 1.0,
            )))
            .observe(recolor_same_kitchen_on::<Pointer<Out>>(Color::srgba(
                1.0, 1.0, 1.0, 1.0,
            )));
    }

    let count = kitchen_query.iter().count();

    if count != 0 {
        *has_run = true;
    }
}

pub fn recolor_same_kitchen_on<E: Debug + Clone + Reflect>(
    color: Color,
) -> impl Fn(Trigger<E>, Query<&Kitchen>, Query<(Entity, &Kitchen)>, Query<&mut Sprite>) {
    move |ev, target_kitchen_query, kitchen_query, mut sprites| {
        let Ok(target_kitchen) = target_kitchen_query.get(ev.target()) else {
            return;
        };

        for (entity, kitchen) in kitchen_query.iter() {
            if kitchen.name == target_kitchen.name {
                if let Ok(mut sprite) = sprites.get_mut(entity) {
                    sprite.color = color;
                }
            }
        }
    }
}

// Mirror implementations
pub fn setup_mirror_hoverable(
    mut commands: Commands,
    mirror_query: Query<Entity, With<Mirror>>,
    mut has_run: Local<bool>,
) {
    if *has_run {
        return;
    }

    for entity in mirror_query.iter() {
        commands
            .entity(entity)
            .insert((Pickable::default(), ZIndex(100)))
            .observe(recolor_same_mirror_on::<Pointer<Over>>(Color::srgb(
                0.0, 1.0, 1.0,
            )))
            .observe(recolor_same_mirror_on::<Pointer<Out>>(Color::srgba(
                1.0, 1.0, 1.0, 1.0,
            )));
    }

    let count = mirror_query.iter().count();

    if count != 0 {
        *has_run = true;
    }
}

pub fn recolor_same_mirror_on<E: Debug + Clone + Reflect>(
    color: Color,
) -> impl Fn(Trigger<E>, Query<&Mirror>, Query<(Entity, &Mirror)>, Query<&mut Sprite>) {
    move |ev, target_mirror_query, mirror_query, mut sprites| {
        let Ok(target_mirror) = target_mirror_query.get(ev.target()) else {
            return;
        };

        for (entity, mirror) in mirror_query.iter() {
            if mirror.name == target_mirror.name {
                if let Ok(mut sprite) = sprites.get_mut(entity) {
                    sprite.color = color;
                }
            }
        }
    }
}

// ComputerDesk implementations
pub fn setup_computer_desk_hoverable(
    mut commands: Commands,
    computer_desk_query: Query<Entity, With<ComputerDesk>>,
    mut has_run: Local<bool>,
) {
    if *has_run {
        return;
    }

    for entity in computer_desk_query.iter() {
        commands
            .entity(entity)
            .insert((Pickable::default(), ZIndex(100)))
            .observe(recolor_same_computer_desk_on::<Pointer<Over>>(Color::srgb(
                0.0, 1.0, 1.0,
            )))
            .observe(recolor_same_computer_desk_on::<Pointer<Out>>(Color::srgba(
                1.0, 1.0, 1.0, 1.0,
            )));
    }

    let count = computer_desk_query.iter().count();

    if count != 0 {
        *has_run = true;
    }
}

pub fn recolor_same_computer_desk_on<E: Debug + Clone + Reflect>(
    color: Color,
) -> impl Fn(Trigger<E>, Query<&ComputerDesk>, Query<(Entity, &ComputerDesk)>, Query<&mut Sprite>) {
    move |ev, target_computer_desk_query, computer_desk_query, mut sprites| {
        let Ok(target_computer_desk) = target_computer_desk_query.get(ev.target()) else {
            return;
        };

        for (entity, computer_desk) in computer_desk_query.iter() {
            if computer_desk.name == target_computer_desk.name {
                if let Ok(mut sprite) = sprites.get_mut(entity) {
                    sprite.color = color;
                }
            }
        }
    }
}

// Couch implementations
pub fn setup_couch_hoverable(
    mut commands: Commands,
    couch_query: Query<Entity, With<Couch>>,
    mut has_run: Local<bool>,
) {
    if *has_run {
        return;
    }

    for entity in couch_query.iter() {
        commands
            .entity(entity)
            .insert((Pickable::default(), ZIndex(100)))
            .observe(recolor_same_couch_on::<Pointer<Over>>(Color::srgb(
                0.0, 1.0, 1.0,
            )))
            .observe(recolor_same_couch_on::<Pointer<Out>>(Color::srgba(
                1.0, 1.0, 1.0, 1.0,
            )));
    }

    let count = couch_query.iter().count();

    if count != 0 {
        *has_run = true;
    }
}

pub fn recolor_same_couch_on<E: Debug + Clone + Reflect>(
    color: Color,
) -> impl Fn(Trigger<E>, Query<&Couch>, Query<(Entity, &Couch)>, Query<&mut Sprite>) {
    move |ev, target_couch_query, couch_query, mut sprites| {
        let Ok(target_couch) = target_couch_query.get(ev.target()) else {
            return;
        };

        for (entity, couch) in couch_query.iter() {
            if couch.name == target_couch.name {
                if let Ok(mut sprite) = sprites.get_mut(entity) {
                    sprite.color = color;
                }
            }
        }
    }
}

// WaterBottle implementations
pub fn setup_water_bottle_hoverable(
    mut commands: Commands,
    water_bottle_query: Query<Entity, With<WaterBottle>>,
    mut has_run: Local<bool>,
) {
    if *has_run {
        return;
    }

    for entity in water_bottle_query.iter() {
        commands
            .entity(entity)
            .insert((Pickable::default(), ZIndex(100)))
            .observe(recolor_same_water_bottle_on::<Pointer<Over>>(Color::srgb(
                0.0, 1.0, 1.0,
            )))
            .observe(recolor_same_water_bottle_on::<Pointer<Out>>(Color::srgba(
                1.0, 1.0, 1.0, 1.0,
            )));
    }

    let count = water_bottle_query.iter().count();

    if count != 0 {
        *has_run = true;
    }
}

pub fn recolor_same_water_bottle_on<E: Debug + Clone + Reflect>(
    color: Color,
) -> impl Fn(Trigger<E>, Query<&WaterBottle>, Query<(Entity, &WaterBottle)>, Query<&mut Sprite>) {
    move |ev, target_water_bottle_query, water_bottle_query, mut sprites| {
        let Ok(target_water_bottle) = target_water_bottle_query.get(ev.target()) else {
            return;
        };

        for (entity, water_bottle) in water_bottle_query.iter() {
            if water_bottle.name == target_water_bottle.name {
                if let Ok(mut sprite) = sprites.get_mut(entity) {
                    sprite.color = color;
                }
            }
        }
    }
}

// Toilet implementations
pub fn setup_toilet_hoverable(
    mut commands: Commands,
    toilet_query: Query<Entity, With<Toilet>>,
    mut has_run: Local<bool>,
) {
    if *has_run {
        return;
    }

    for entity in toilet_query.iter() {
        commands
            .entity(entity)
            .insert((Pickable::default(), ZIndex(100)))
            .observe(recolor_same_toilet_on::<Pointer<Over>>(Color::srgb(
                0.0, 1.0, 1.0,
            )))
            .observe(recolor_same_toilet_on::<Pointer<Out>>(Color::srgba(
                1.0, 1.0, 1.0, 1.0,
            )));
    }

    let count = toilet_query.iter().count();

    if count != 0 {
        *has_run = true;
    }
}

pub fn recolor_same_toilet_on<E: Debug + Clone + Reflect>(
    color: Color,
) -> impl Fn(Trigger<E>, Query<&Toilet>, Query<(Entity, &Toilet)>, Query<&mut Sprite>) {
    move |ev, target_toilet_query, toilet_query, mut sprites| {
        let Ok(target_toilet) = target_toilet_query.get(ev.target()) else {
            return;
        };

        for (entity, toilet) in toilet_query.iter() {
            if toilet.name == target_toilet.name {
                if let Ok(mut sprite) = sprites.get_mut(entity) {
                    sprite.color = color;
                }
            }
        }
    }
}

// Sink implementations
pub fn setup_sink_hoverable(
    mut commands: Commands,
    sink_query: Query<Entity, With<Sink>>,
    mut has_run: Local<bool>,
) {
    if *has_run {
        return;
    }

    for entity in sink_query.iter() {
        commands
            .entity(entity)
            .insert((Pickable::default(), ZIndex(100)))
            .observe(recolor_same_sink_on::<Pointer<Over>>(Color::srgb(
                0.0, 1.0, 1.0,
            )))
            .observe(recolor_same_sink_on::<Pointer<Out>>(Color::srgba(
                1.0, 1.0, 1.0, 1.0,
            )));
    }

    let count = sink_query.iter().count();

    if count != 0 {
        *has_run = true;
    }
}

pub fn recolor_same_sink_on<E: Debug + Clone + Reflect>(
    color: Color,
) -> impl Fn(Trigger<E>, Query<&Sink>, Query<(Entity, &Sink)>, Query<&mut Sprite>) {
    move |ev, target_sink_query, sink_query, mut sprites| {
        let Ok(target_sink) = target_sink_query.get(ev.target()) else {
            return;
        };

        for (entity, sink) in sink_query.iter() {
            if sink.name == target_sink.name {
                if let Ok(mut sprite) = sprites.get_mut(entity) {
                    sprite.color = color;
                }
            }
        }
    }
}
