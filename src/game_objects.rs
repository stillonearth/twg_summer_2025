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

#[derive(Component)]
pub struct GameObjectName;

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
            )))
            .observe(show_bed_name_on_hover)
            .observe(hide_bed_name_on_unhover);
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

pub fn show_bed_name_on_hover(
    trigger: Trigger<Pointer<Over>>,
    mut commands: Commands,
    target_bed_query: Query<&Bed>,
    bed_query: Query<(Entity, &Bed, &Transform)>,
    existing_display: Query<Entity, With<GameObjectName>>,
) {
    // Remove any existing name display
    for entity in existing_display.iter() {
        commands.entity(entity).despawn();
    }

    // Get the name of the target bed
    let Ok(target_bed) = target_bed_query.get(trigger.target()) else {
        return;
    };

    // Find all beds with the same name and calculate center position
    let same_name_beds: Vec<_> = bed_query
        .iter()
        .filter(|(_, bed, _)| bed.name == target_bed.name)
        .collect();

    if same_name_beds.is_empty() {
        return;
    }

    // Calculate center position of all beds with same name
    let center = same_name_beds
        .iter()
        .map(|(_, _, transform)| transform.translation)
        .fold(Vec3::ZERO, |acc, pos| acc + pos)
        / same_name_beds.len() as f32;

    println!("~~");

    // Spawn text entity at center position
    commands.spawn((
        GameObjectName,
        Text::new(target_bed.name.clone()),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_translation(center + Vec3::new(0.0, 0.0, 10.0)),
    ));
}

pub fn hide_bed_name_on_unhover(
    _trigger: Trigger<Pointer<Out>>,
    mut commands: Commands,
    existing_display: Query<Entity, With<GameObjectName>>,
) {
    // Remove the name display
    for entity in existing_display.iter() {
        commands.entity(entity).despawn();
    }
}
