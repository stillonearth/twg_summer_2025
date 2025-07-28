use crate::cards::ActivityCard;
use crate::logic::{ActionCompletedEvent, CutsceneEndEvent, CutsceneStartEvent, GamePhaseState};
use crate::navigation::{GridPos, MovePlayerCommand, TileSize};
use crate::sprites::{
    get_animation_indices, AnimatedCharacterSprite, AnimatedCharacterType, AnimationDirection,
    AnimationState, AnimationTimer, AnimationType, CharacterAnimation, PLAYER_ASSET_SHEET_1,
    SHEET_1_COLUMNS, SHEET_1_ROWS,
};
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_la_mesa::Card;

const MOVE_SPEED: f32 = 200.;

// Add the new event
#[derive(Event)]
pub struct PlayerDestinationReachedEvent {}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerDestinationReachedEvent>()
            .add_systems(
                Update,
                (
                    move_player_from_command,
                    move_player_along_path,
                    handle_cutscene_start,
                    handle_cutscene_end,
                    handle_player_destination_reached,
                ),
            );
    }
}

#[derive(Default, Clone, Component)]
pub struct PlayerMarker;

#[derive(Default, Clone, Component)]
pub struct NonVisualNovelElement;

#[derive(Component, Default)]
pub struct PlayerMovement {
    pub path: Vec<GridPos>,
    pub current_target_index: usize,
    pub is_moving: bool,
}

pub fn move_player_from_command(
    mut move_events: EventReader<MovePlayerCommand>,
    mut player_query: Query<&mut PlayerMovement, With<PlayerMarker>>,
) {
    for event in move_events.read() {
        if let Ok(mut player_movement) = player_query.single_mut() {
            // Skip the first position in the path (current position)
            let path = if event.path.len() > 1 {
                event.path[1..].to_vec()
            } else {
                event.path.clone()
            };

            player_movement.path = path;
            player_movement.current_target_index = 0;
            player_movement.is_moving = !player_movement.path.is_empty();
        }
    }
}

pub fn move_player_along_path(
    mut player_query: Query<
        (
            Entity,
            &mut Transform,
            &mut LinearVelocity,
            &mut PlayerMovement,
        ),
        With<PlayerMarker>,
    >,
    mut animation_query: Query<&mut CharacterAnimation, With<AnimatedCharacterSprite>>,
    mut destination_events: EventWriter<PlayerDestinationReachedEvent>,
    tile_size: Res<TileSize>,
) {
    for (_, transform, mut rb_vel, mut player_movement) in player_query.iter_mut() {
        if !player_movement.is_moving || player_movement.path.is_empty() {
            rb_vel.0 = Vec2::ZERO;
            // Set animation to stand when not moving
            for mut character_animation in &mut animation_query {
                character_animation.animation_type = AnimationType::Stand;
            }
            continue;
        }

        let current_target = player_movement.path[player_movement.current_target_index];
        let target_world_pos = Vec3::new(
            current_target.x as f32 * tile_size.0 + tile_size.0 / 2.0,
            current_target.y as f32 * tile_size.0 + tile_size.0 / 2.0,
            transform.translation.z, // Keep the same Z
        );

        let direction = (target_world_pos - transform.translation).truncate();
        let distance = direction.length();

        // Check if we've reached the current target
        if distance < 5.0 {
            // Small threshold for "reached"
            player_movement.current_target_index += 1;

            // Check if we've reached the end of the path
            if player_movement.current_target_index >= player_movement.path.len() {
                player_movement.is_moving = false;
                rb_vel.0 = Vec2::ZERO;

                // Set animation to stand when path is complete
                for mut character_animation in &mut animation_query {
                    character_animation.animation_type = AnimationType::Stand;
                }

                // Send the destination reached event
                destination_events.write(PlayerDestinationReachedEvent {});
                continue;
            }
        }

        // Move towards the current target
        let normalized_direction = direction.normalize_or_zero();
        rb_vel.0 = normalized_direction * MOVE_SPEED;

        // Update animation based on movement direction
        for mut character_animation in &mut animation_query {
            character_animation.animation_type = AnimationType::Walk;

            // Determine animation direction based on movement
            if normalized_direction.x.abs() > normalized_direction.y.abs() {
                if normalized_direction.x > 0.0 {
                    character_animation.direction = AnimationDirection::Right;
                } else {
                    character_animation.direction = AnimationDirection::Left;
                }
            } else if normalized_direction.y > 0.0 {
                character_animation.direction = AnimationDirection::Up;
            } else {
                character_animation.direction = AnimationDirection::Down;
            }
        }
    }
}

pub fn spawn_player_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load(PLAYER_ASSET_SHEET_1);
    let layout =
        TextureAtlasLayout::from_grid(UVec2::splat(64), SHEET_1_COLUMNS, SHEET_1_ROWS, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let character_animation = CharacterAnimation {
        state: AnimationState::Idle,
        direction: AnimationDirection::Down,
        animation_type: AnimationType::Stand,
    };

    let animation_indices = get_animation_indices(
        character_animation.animation_type,
        character_animation.direction,
    );

    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
        ),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        character_animation,
        AnimatedCharacterSprite {
            animated_character_type: AnimatedCharacterType::Player,
        },
        RigidBody::Dynamic,
        PlayerMarker,
        Name::new("Player"),
        PlayerMovement::default(),
        Collider::circle(10.),
        LockedAxes::ROTATION_LOCKED,
    ));
}

fn handle_cutscene_start(
    mut commands: Commands,
    mut er_cutscene_start: EventReader<CutsceneStartEvent>,
    q_players: Query<(Entity, &PlayerMarker)>,
) {
    for _ in er_cutscene_start.read() {
        for (entity, _) in q_players.iter() {
            commands.entity(entity).insert(Visibility::Hidden);
        }
    }
}

fn handle_cutscene_end(
    mut commands: Commands,
    mut er_cutscene_start: EventReader<CutsceneEndEvent>,
    q_players: Query<(Entity, &PlayerMarker)>,
) {
    for _ in er_cutscene_start.read() {
        for (entity, _) in q_players.iter() {
            commands.entity(entity).insert(Visibility::Inherited);
        }
    }
}

// Example system to handle the destination reached event
pub fn handle_player_destination_reached(
    mut destination_events: EventReader<PlayerDestinationReachedEvent>,
    mut ew_action_completed: EventWriter<ActionCompletedEvent>,
    phase_state: Res<GamePhaseState>,
    q_cards: Query<(Entity, &Card<ActivityCard>)>,
) {
    for _ in destination_events.read() {
        if let Some(selected_card_id) = phase_state.selected_card_id
            && let Some(card) = q_cards
                .iter()
                .find(|(_, card)| card.data.id == selected_card_id)
        {
            ew_action_completed.write(ActionCompletedEvent {
                card_played: card.1.data.clone(),
            });
        }
    }
}
