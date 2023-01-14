use std::{net::UdpSocket, time::SystemTime};

use bevy::prelude::*;
use bevy_renet::*;

use crate::{GameAssets, GameState};
use shared::PROTOCOL_ID;

const BUTTON_MARGIN: UiRect = UiRect {
    top: Val::Px(10.0),
    bottom: Val::Px(10.0),
    left: Val::Px(10.0),
    right: Val::Px(10.0),
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup_main_menu));
    }
}

fn create_renet_config() -> renet::RenetClient {
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let server_addr = "127.0.0.1:5678".parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();

    let config = renet::RenetConnectionConfig::default();

    let authentication = renet::ClientAuthentication::Unsecure {
        protocol_id: PROTOCOL_ID,
        client_id: current_time.as_millis() as u64,
        server_addr,
        user_data: None,
    };

    renet::RenetClient::new(current_time, socket, config, authentication).unwrap()
}

fn setup_main_menu(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                padding: UiRect::all(Val::Percent(1.)),
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|node| {
            node.spawn(TextBundle::from_section(
                "Cool Survival Game",
                TextStyle {
                    font_size: 40.0,
                    color: Color::WHITE,
                    font: game_assets.font.clone(),
                },
            ));

            node.spawn(NodeBundle {
                style: Style {
                    margin: UiRect {
                        top: Val::Percent(20.0),
                        bottom: Val::Percent(20.0),
                        ..Default::default()
                    },
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|center_node| {
                center_node
                    .spawn(ButtonBundle {
                        background_color: BackgroundColor(Color::rgba(0.1, 0.1, 0.1, 1.0)),
                        style: Style {
                            padding: BUTTON_MARGIN,
                            margin: BUTTON_MARGIN,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|button| {
                        button.spawn(TextBundle::from_section(
                            "Connect to Localhost",
                            TextStyle {
                                font_size: 20.0,
                                color: Color::WHITE,
                                font: game_assets.font.clone(),
                            },
                        ));
                    });

                center_node
                    .spawn(ButtonBundle {
                        background_color: BackgroundColor(Color::rgba(0.1, 0.1, 0.1, 1.0)),
                        style: Style {
                            padding: BUTTON_MARGIN,
                            margin: BUTTON_MARGIN,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|button| {
                        button.spawn(TextBundle::from_section(
                            "Quit",
                            TextStyle {
                                font_size: 20.0,
                                color: Color::WHITE,
                                font: game_assets.font.clone(),
                            },
                        ));
                    });
            });
        });
}
