use bevy::{prelude::*, render::view::RenderLayers};


pub const SHEET_1_COLUMNS: u32 = 13;
pub const SHEET_1_ROWS: u32 = 21;
pub const N_FRAMES_WALK: usize = 8;

pub const PLAYER_ASSET_SHEET_1: &str = "character.png";
pub const LAYER_SPRITES: usize = 1;

#[derive(Copy, Clone, Reflect, Default, Debug, PartialEq, Eq)]
pub enum AnimatedCharacterType {
    #[default]
    Player,
}

#[derive(Copy, Clone, Component, Reflect, Default)]
pub struct AnimatedCharacterSprite {
    pub animated_character_type: AnimatedCharacterType,
}

#[allow(dead_code)]
#[derive(Component, Clone, Default, Debug, Reflect)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Clone, Default, Debug, Reflect)]
pub enum AnimationState {
    #[default]
    Idle,
    // Run,
}

#[derive(Clone, Default, Copy, PartialEq, Debug, Reflect)]
pub enum AnimationDirection {
    #[default]
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Default, Copy, PartialEq, Debug, Reflect)]
pub enum AnimationType {
    Walk,
    #[default]
    Stand,
}

#[derive(Component, Clone, Default, Debug, Reflect)]
pub struct CharacterAnimation {
    pub state: AnimationState,
    pub direction: AnimationDirection,
    pub animation_type: AnimationType,
}

#[derive(Component, Deref, DerefMut, Clone, Default, Reflect)]
pub struct AnimationTimer(pub Timer);

#[allow(clippy::erasing_op)]
pub fn get_animation_indices(
    animation_type: AnimationType,
    animation_direction: AnimationDirection,
) -> AnimationIndices {
    let mut first = 0;
    let mut last = 0;

    // Walk animations
    if animation_type == AnimationType::Walk && animation_direction == AnimationDirection::Right {
        first = SHEET_1_COLUMNS as usize * 11 + 1;
        last = SHEET_1_COLUMNS as usize * 11 + N_FRAMES_WALK;
    }
    if animation_type == AnimationType::Walk && animation_direction == AnimationDirection::Left {
        first = SHEET_1_COLUMNS as usize * 9 + 1;
        last = SHEET_1_COLUMNS as usize * 9 + N_FRAMES_WALK;
    }
    if animation_type == AnimationType::Walk && animation_direction == AnimationDirection::Up {
        first = SHEET_1_COLUMNS as usize * 8 + 1;
        last = SHEET_1_COLUMNS as usize * 8 + N_FRAMES_WALK;
    }
    if animation_type == AnimationType::Walk && animation_direction == AnimationDirection::Down {
        first = SHEET_1_COLUMNS as usize * 10 + 1;
        last = SHEET_1_COLUMNS as usize * 10 + N_FRAMES_WALK;
    }

    // Stand animations
    if animation_type == AnimationType::Stand && animation_direction == AnimationDirection::Right {
        first = SHEET_1_COLUMNS as usize * 11;
        last = first;
    }
    if animation_type == AnimationType::Stand && animation_direction == AnimationDirection::Left {
        first = SHEET_1_COLUMNS as usize * 9;
        last = first;
    }
    if animation_type == AnimationType::Stand && animation_direction == AnimationDirection::Up {
        first = SHEET_1_COLUMNS as usize * 8;
        last = first;
    }
    if animation_type == AnimationType::Stand && animation_direction == AnimationDirection::Down {
        first = SHEET_1_COLUMNS as usize * 10;
        last = first;
    }

    AnimationIndices { first, last }
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if (atlas.index >= indices.last) || (atlas.index < indices.first) {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}

// System to update animation indices when character animation changes
pub fn update_animation_indices(
    mut query: Query<(&CharacterAnimation, &mut AnimationIndices), Changed<CharacterAnimation>>,
) {
    for (character_animation, mut animation_indices) in &mut query {
        let new_indices = get_animation_indices(
            character_animation.animation_type,
            character_animation.direction,
        );
        *animation_indices = new_indices;
    }
}

pub fn add_render_layers_to_sprites(
    mut commands: Commands,
    sprites_without_layers: Query<Entity, (With<Sprite>, Without<RenderLayers>)>,
    mut has_run: Local<bool>,
) {
    if *has_run {
        return;
    }

    for entity in sprites_without_layers.iter() {
        commands
            .entity(entity)
            .insert(RenderLayers::layer(LAYER_SPRITES));
    }

    let count = sprites_without_layers.iter().count();

    if count != 0 {
        *has_run = true;
    }
}
