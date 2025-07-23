use avian2d::prelude::*;
use bevy::{input::common_conditions::input_toggle_active, prelude::*, render::view::RenderLayers};
use bevy_ecs_tiled::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use crate::{sprites::LAYER_SPRITES, ui::LAYER_UI};

mod collisions;
mod debug;
mod game_objects;
mod navigation;
mod player;
mod sprites;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ui::HikikomoriUIPlugin)
        .add_plugins(game_objects::GameObjectsPlugin)
        .add_plugins(TiledMapPlugin::default())
        .add_plugins(TiledPhysicsPlugin::<TiledPhysicsAvianBackend>::default())
        .add_plugins(PhysicsPlugins::default().with_length_unit(100.0))
        .add_plugins((
            EguiPlugin {
                enable_multipass_for_primary_context: true,
            },
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        ))
        .add_plugins(navigation::NavigationGridPlugin {})
        .add_systems(Startup, (startup,))
        .add_systems(
            Update,
            (
                player::move_player,
                collisions::check_nearest_object,
                debug::debug_draw_system,
                sprites::animate_sprite,
                sprites::update_animation_indices,
                ui::example_game_loop,
                game_objects::setup_bed_hoverable,
                game_objects::setup_bath_hoverable,
                game_objects::setup_kitchen_hoverable,
                game_objects::setup_toilet_hoverable,
                game_objects::setup_sink_hoverable,
                game_objects::setup_mirror_hoverable,
                game_objects::setup_computer_desk_hoverable,
                game_objects::setup_couch_hoverable,
                game_objects::setup_water_bottle_hoverable,
                sprites::add_render_layers_to_sprites,
            ),
        )
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, IsDefaultUiCamera));

    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        RenderLayers::layer(LAYER_SPRITES),
    ));

    let map_handle: Handle<TiledMap> = asset_server.load("my_room.tmx");
    commands
        .spawn((TiledMapHandle(map_handle), TilemapAnchor::Center))
        .observe(
            |_: Trigger<TiledMapCreated>,
             commands: Commands,
             asset_server: Res<AssetServer>,
             texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>| {
                sprites::spawn_character_sprite(commands, asset_server, texture_atlas_layouts);
            },
        )
        .observe(
            |trigger: Trigger<TiledColliderCreated>, mut commands: Commands| {
                commands.entity(trigger.entity).insert((RigidBody::Static,));
            },
        );
}
