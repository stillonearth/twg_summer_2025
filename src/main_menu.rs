use std::time::Duration;

use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy_hui::prelude::*;
use bevy_kira_audio::*;

use crate::AppState;

pub struct MainMenuPlugin;

const SHADER_ASSET_PATH: &str = "shaders/balatro.wgsl";

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartCardAnimation>()
            .add_plugins(MaterialPlugin::<ShaderMaterial>::default())
            .add_systems(OnEnter(AppState::MainMenu), show_menu)
            .add_systems(OnExit(AppState::MainMenu), despawn_menu);
    }
}

#[derive(Component)]
pub struct MainMenuResource {}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ShaderMaterial {}

#[derive(Event)]
pub struct StartCardAnimation {}

impl Material for ShaderMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}

pub fn show_menu(
    audio: Res<Audio>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut custom_materials: ResMut<Assets<ShaderMaterial>>,
    asset_server: Res<AssetServer>,
    mut html_funcs: HtmlFunctions,
    mut q_cameras: Query<(Entity, &mut Camera, &Camera2d)>,
) {
    // table
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default())),
        MeshMaterial3d(custom_materials.add(ShaderMaterial {})),
        Transform::from_translation(Vec3::new(0.0, -1.0, 0.0)).with_scale(Vec3::ONE * 50.0),
        ZIndex(2),
        MainMenuResource {},
    ));

    // menu
    commands.spawn((
        HtmlNode(asset_server.load("menu/main_menu.html")),
        TemplateProperties::default(),
        MainMenuResource {},
    ));

    audio
        .play(asset_server.load("music/echo_code.mp3"))
        .loop_from(0.5)
        .fade_in(AudioTween::new(
            Duration::from_secs(2),
            AudioEasing::OutPowi(2),
        ));

    // main menu handler
    html_funcs.register(
        "start_game",
        |In(_), mut app_state: ResMut<NextState<AppState>>| {
            app_state.set(AppState::Game);
        },
    );

    for (_, mut camera, _) in q_cameras.iter_mut() {
        camera.order = 2;
    }
}

pub fn despawn_menu(
    mut commands: Commands,
    q_main_menu_entities: Query<(Entity, &MainMenuResource)>,
    audio: Res<Audio>,
) {
    for (entity, _) in q_main_menu_entities.iter() {
        commands.entity(entity).despawn();
    }

    audio.stop();
}
