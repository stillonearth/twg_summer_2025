use bevy::prelude::*;

use bevy_hui::prelude::*;
use bevy_novel::events::EventSwitchNextNode;

use crate::{
    AppState,
    logic::{CutsceneEndEvent, CutsceneStartEvent},
};

pub struct GameMenuPlugin;

impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (show_menu, despawn_menu, render_ui).run_if(in_state(AppState::Game)),
        )
        .add_event::<CutsceneStartEvent>()
        .add_event::<CutsceneEndEvent>()
        .add_event::<EventRenderUI>();
    }
}

#[derive(Component)]
pub struct GameMenu;

/// Despawn previous menu template and render a new one
#[derive(Event, PartialEq, Eq, Default, Debug)]
pub enum EventRenderUI {
    #[default]
    Novel,
}

#[derive(Event, PartialEq, Eq, Default, Debug)]
pub struct NarrativeMenuSettings {
    pub show_advance_button: bool,
}

pub fn show_menu(
    mut commands: Commands,
    mut html_funcs: HtmlFunctions,
    asset_server: Res<AssetServer>,
    mut er_cutscene_start: EventReader<CutsceneStartEvent>,
    mut ew_render_ui: EventWriter<EventRenderUI>,
) {
    for _ in er_cutscene_start.read() {
        // Spawn the initial menu when cutscene starts
        commands.spawn((
            HtmlNode(asset_server.load("menu/novel_menu.html")),
            TemplateProperties::default(),
            GameMenu,
        ));

        // Send render UI event for Novel menu
        ew_render_ui.write(EventRenderUI::Novel);

        // game menu button handlers
        html_funcs.register(
            "advance",
            |In(_), mut ew_switch_next_node: EventWriter<EventSwitchNextNode>| {
                ew_switch_next_node.write(EventSwitchNextNode {});
            },
        );
    }
}

fn despawn_menu(
    mut commands: Commands,
    q_main_menu_entities: Query<(Entity, &GameMenu)>,
    mut er_cutscene_end: EventReader<CutsceneEndEvent>,
) {
    for _ in er_cutscene_end.read() {
        for (entity, _) in q_main_menu_entities.iter() {
            commands.entity(entity).despawn();
        }
    }
}

fn render_ui(
    mut commands: Commands,
    mut er_render_ui: EventReader<EventRenderUI>,
    q_game_menu: Query<(Entity, &GameMenu)>,
    asset_server: Res<AssetServer>,
) {
    for event in er_render_ui.read() {
        // Despawn existing menu entities
        for (entity, _) in q_game_menu.iter() {
            commands.entity(entity).despawn();
        }

        match event {
            EventRenderUI::Novel => {
                commands.spawn((
                    HtmlNode(asset_server.load("menu/novel_menu.html")),
                    TemplateProperties::default(),
                    GameMenu,
                    Name::new("novel menu"),
                ));
            }
        }
    }
}
