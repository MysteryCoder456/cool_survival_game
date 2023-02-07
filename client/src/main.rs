use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use bevy_renet::*;
use iyes_loopless::prelude::*;

mod connecting_screen;
mod main_menu;

use connecting_screen::ConnectingScreenPlugin;
use main_menu::MainMenuPlugin;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    Connecting,
    Game,
}

#[derive(Resource)]
pub struct GameAssets {
    font: Handle<Font>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Cool Survival Game".to_owned(),
                ..Default::default()
            },
            ..Default::default()
        }))
        // 3rd Party Plugins
        .add_plugin(RenetClientPlugin::default())
        // 1st Party Plugins
        .add_plugin(MainMenuPlugin)
        .add_plugin(ConnectingScreenPlugin)
        .add_state(GameState::MainMenu)
        .add_startup_system_to_stage(StartupStage::PreStartup, setup)
        .add_system(handle_client_connection_state.run_if_resource_exists::<renet::RenetClient>())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::DARK_GREEN),
        },
        ..Default::default()
    });

    let game_assets = GameAssets {
        font: asset_server.load("fonts/HackNerdFont.ttf"),
    };
    commands.insert_resource(game_assets);
}

fn handle_client_connection_state(
    mut commands: Commands,
    mut game_state: ResMut<State<GameState>>,
    client: Res<renet::RenetClient>,
) {
    if !client.is_changed() || client.is_added() {
        return;
    }

    let new_state = if client.is_connected() {
        GameState::Game
    } else {
        commands.remove_resource::<renet::RenetClient>();
        GameState::MainMenu
    };

    let _ = game_state.set(new_state);
}
