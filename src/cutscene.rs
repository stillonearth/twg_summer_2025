use bevy::prelude::*;
use bevy_novel::{NovelText, events::EventNovelEnd, rpy_asset_loader::Rpy};

use crate::{
    AppState,
    endgame::EndGameEvent,
    logic::{CutsceneEndEvent, CutsceneStartEvent},
};

pub struct CutscenePlugin;

impl Plugin for CutscenePlugin {
    fn build(&self, app: &mut App) {
        // Initialize app state and resources
        app.add_systems(OnEnter(AppState::Game), load_scenario)
            .add_systems(
                Update,
                (start_visual_novel, handle_novel_end, handle_game_over)
                    .run_if(in_state(AppState::Game)),
            );
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct ScenarioHandle(Vec<Handle<Rpy>>);

fn load_scenario(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scenario_handle = ScenarioHandle(vec![
        asset_server.load("scenarios/0.rpy"),
        asset_server.load("scenarios/1.rpy"),
        asset_server.load("scenarios/2.rpy"),
        asset_server.load("scenarios/3.rpy"),
        asset_server.load("scenarios/4.rpy"),
        asset_server.load("scenarios/5.rpy"),
        asset_server.load("scenarios/6.rpy"),
        asset_server.load("scenarios/7.rpy"),
        asset_server.load("scenarios/8.rpy"),
        asset_server.load("scenarios/9.rpy"),
        asset_server.load("scenarios/10.rpy"),
        asset_server.load("scenarios/11.rpy"),
        asset_server.load("scenarios/12.rpy"),
        asset_server.load("scenarios/13.rpy"),
        asset_server.load("scenarios/14.rpy"),
        asset_server.load("scenarios/15.rpy"),
        asset_server.load("scenarios/16.rpy"),
        asset_server.load("scenarios/17.rpy"),
        asset_server.load("scenarios/18.rpy"),
        asset_server.load("scenarios/19.rpy"),
        asset_server.load("scenarios/20.rpy"),
        asset_server.load("scenarios/21.rpy"),
        asset_server.load("scenarios/22.rpy"),
        asset_server.load("scenarios/23.rpy"),
        asset_server.load("scenarios/24.rpy"),
        asset_server.load("scenarios/25.rpy"),
        asset_server.load("scenarios/26.rpy"),
        asset_server.load("scenarios/27.rpy"),
        asset_server.load("scenarios/28.rpy"),
        asset_server.load("scenarios/29.rpy"),
        asset_server.load("scenarios/30.rpy"),
        asset_server.load("scenarios/31.rpy"),
        asset_server.load("scenarios/32.rpy"),
        asset_server.load("scenarios/33.rpy"),
        asset_server.load("scenarios/34.rpy"),
    ]);
    commands.insert_resource(scenario_handle);
}

pub fn start_visual_novel(
    mut er_cutscene_start: EventReader<CutsceneStartEvent>,
    mut q_novel_text: Query<(Entity, &mut Node, &NovelText)>,
) {
    for _ in er_cutscene_start.read() {
        for (_, mut node, _) in q_novel_text.iter_mut() {
            node.left = Val::Percent(20.0);
            node.margin = UiRect::new(Val::Px(20.0), Val::Px(0.0), Val::Px(0.0), Val::Px(0.0));
        }
    }
}

pub fn handle_novel_end(
    mut er_novel_end: EventReader<EventNovelEnd>,
    mut ew_cutscene_end: EventWriter<CutsceneEndEvent>,
) {
    for _ in er_novel_end.read() {
        ew_cutscene_end.write(CutsceneEndEvent {});
    }
}

pub fn handle_game_over(mut er_game_over: EventReader<EndGameEvent>) {
    for e in er_game_over.read() {
        println!("endgame scenario: {}", e.scenario_id);
    }
}
