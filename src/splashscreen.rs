use crate::AppState;
use bevy::prelude::*;

#[derive(Component)]
struct Splashscreen;

#[derive(Resource)]
struct SplashTimer {
    timer: Timer,
    current_splash: u8, // 1 for first splash, 2 for second splash
}

pub struct SplashscreenPlugin;

impl Plugin for SplashscreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnExit(AppState::Splashscreen),
            |mut commands: Commands, q_menu_components: Query<(Entity, &Splashscreen)>| {
                for (e, _) in q_menu_components.iter() {
                    commands.entity(e).despawn();
                }
            },
        )
        .add_systems(OnEnter(AppState::Splashscreen), setup_splashscreen)
        .add_systems(
            Update,
            update_splashscreen.run_if(in_state(AppState::Splashscreen)),
        );
    }
}

fn setup_splashscreen(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Initialize the timer resource
    commands.insert_resource(SplashTimer {
        timer: Timer::from_seconds(5.0, TimerMode::Once),
        current_splash: 1,
    });

    // Spawn the first splash screen
    spawn_splash(&mut commands, &asset_server, "splash-1.png");
}

fn spawn_splash(commands: &mut Commands, asset_server: &Res<AssetServer>, image_path: &str) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            Splashscreen,
            ZIndex(0),
            Name::new("splashscreen image container"),
        ))
        .with_children(|parent| {
            parent.spawn((
                Sprite::from_image(asset_server.load(image_path)),
                Transform::from_scale(Vec3::ONE * 1.0),
            ));
        });
}

fn update_splashscreen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut splash_timer: ResMut<SplashTimer>,
    mut app_state: ResMut<NextState<AppState>>,
    q_splash_components: Query<(Entity, &Splashscreen)>,
) {
    splash_timer.timer.tick(time.delta());

    if splash_timer.timer.finished() {
        // Despawn current splash screen
        for (entity, _) in q_splash_components.iter() {
            commands.entity(entity).despawn();
        }

        match splash_timer.current_splash {
            1 => {
                // Show second splash screen
                spawn_splash(&mut commands, &asset_server, "splash-2.png");
                splash_timer.current_splash = 2;
                splash_timer.timer = Timer::from_seconds(5.0, TimerMode::Once);

                // Both splash screens are done, transition to next state
                app_state.set(AppState::MainMenu);
                commands.remove_resource::<SplashTimer>();
            }
            2 => {}
            _ => {}
        }
    }
}
