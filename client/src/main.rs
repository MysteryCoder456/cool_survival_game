use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use bevy_renet::*;

mod connecting_screen;
mod main_game;
mod main_menu;

use connecting_screen::ConnectingScreenPlugin;
use main_game::MainGamePlugin;
use main_menu::MainMenuPlugin;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    Connecting,
    Game,
}

#[derive(Resource)]
pub struct UIAssets {
    font: Handle<Font>,
}

#[derive(Component)]
pub struct MainCamera {
    pub speed: f32,
    pub follow_distance: f32,
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
        .add_plugin(MainGamePlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(ConnectingScreenPlugin)
        .add_state(GameState::MainMenu)
        .add_startup_system_to_stage(StartupStage::PreStartup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::DARK_GREEN),
            },
            ..Default::default()
        },
        MainCamera {
            speed: 400.0,
            follow_distance: 150.0,
        },
    ));

    let ui_assets = UIAssets {
        font: asset_server.load("fonts/HackNerdFont.ttf"),
    };
    commands.insert_resource(ui_assets);
}
