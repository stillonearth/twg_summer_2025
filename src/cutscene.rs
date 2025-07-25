use bevy::prelude::*;
use bevy_novel::{events::EventNovelEnd, rpy_asset_loader::Rpy, NovelText};

use crate::logic::{CutsceneEndEvent, CutsceneStartEvent};

pub struct CutscenePlugin;

impl Plugin for CutscenePlugin {
    fn build(&self, app: &mut App) {
        // Initialize app state and resources
        app.add_systems(Startup, load_scenario)
            .add_systems(Update, (start_visual_novel, handle_novel_end));
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct ScenarioHandle(Handle<Rpy>);

fn load_scenario(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scenario_handle = ScenarioHandle(asset_server.load("script.rpy"));
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
