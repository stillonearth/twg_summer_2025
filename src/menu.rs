use bevy::prelude::*;

use bevy_hui::prelude::*;
use bevy_kira_audio::*;
use bevy_novel::events::EventSwitchNextNode;

use crate::logic::{CutsceneEndEvent, CutsceneStartEvent};

pub struct GameMenuPlugin;

impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (show_menu, despawn_menu, render_ui, refresh_ui))
            .add_event::<CutsceneStartEvent>()
            .add_event::<CutsceneEndEvent>()
            .add_event::<EventRefreshUI>()
            .add_event::<EventRenderUI>();
    }
}

#[derive(Component)]
pub struct GameMenu;

/// Update Menu display variables
#[derive(Event, PartialEq, Eq)]
pub enum EventRefreshUI {
    NovelMenu(String),
    LoadingMenu,
    GameOver,
}

/// Despawn previous menu template and render a new one
#[derive(Event, PartialEq, Eq, Default, Debug)]
pub enum EventRenderUI {
    #[default]
    Novel,
    Loading,
    Narrative,
    GameOver,
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
    audio: Res<Audio>,
    mut er_cutscene_end: EventReader<CutsceneEndEvent>,
) {
    for _ in er_cutscene_end.read() {
        for (entity, _) in q_main_menu_entities.iter() {
            commands.entity(entity).despawn();
        }

        audio.stop();
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
            EventRenderUI::Loading => {
                commands.spawn((
                    HtmlNode(asset_server.load("menu/loading_menu.html")),
                    TemplateProperties::default(),
                    GameMenu,
                    Name::new("loading menu"),
                ));
            }
            EventRenderUI::Narrative => {
                commands.spawn((
                    HtmlNode(asset_server.load("menu/narrative_menu.html")),
                    TemplateProperties::default(),
                    GameMenu,
                    Name::new("narrative menu"),
                ));
            }
            EventRenderUI::GameOver => {
                commands.spawn((
                    HtmlNode(asset_server.load("menu/game_over.html")),
                    TemplateProperties::default(),
                    GameMenu,
                    Name::new("game over menu"),
                ));
            }
        }
    }
}

fn refresh_ui(
    mut er_refresh_ui: EventReader<EventRefreshUI>,
    mut q_text_labels: Query<(Entity, &mut Text, &Tags)>,
    mut q_nodes: Query<(Entity, &mut Node, &Tags)>,
    mut style: Query<&mut HtmlStyle>,
) {
    for event in er_refresh_ui.read() {
        match event {
            EventRefreshUI::NovelMenu(title) => {
                for (_, mut text, tags) in q_text_labels.iter_mut() {
                    if let Some(marker) = tags.get("marker")
                        && marker == "text_title"
                    {
                        *text = Text::new(title);
                    }
                }
            }
            EventRefreshUI::LoadingMenu => {
                for (entity, mut node, tags) in q_nodes.iter_mut() {
                    if let Some(marker) = tags.get("marker")
                        && marker == "button_advance"
                    {
                        node.display = Display::Flex;

                        if let Ok(mut style) = style.get_mut(entity) {
                            style.computed.node.display = node.display;
                        }
                    }
                }
            }
            EventRefreshUI::GameOver => {
                for (_, mut text, tags) in q_text_labels.iter_mut() {
                    if let Some(marker) = tags.get("marker")
                        && marker == "text_minting_status"
                    {
                        *text = Text::new(format!("Game Over"));
                    }
                }
            }
        }
    }
}
