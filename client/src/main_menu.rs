use std::{
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy::{app::AppExit, prelude::*};
use bevy_renet::*;

use crate::{GameAssets, GameState};
use shared::PROTOCOL_ID;

const BUTTON_MARGIN: UiRect = UiRect {
    top: Val::Px(10.0),
    bottom: Val::Px(10.0),
    left: Val::Px(10.0),
    right: Val::Px(10.0),
};

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
enum Button {
    Connect,
    Quit,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup_main_menu))
            .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(destroy_main_menu))
            .add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(handle_buttons));
    }
}

fn create_renet_client() -> renet::RenetClient {
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    // TODO: Implement entering a custom server address
    let server_addr: SocketAddr = "127.0.0.1:5678".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();

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
        .spawn((
            NodeBundle {
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
            },
            MainMenu,
        ))
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
                    .spawn((
                        ButtonBundle {
                            background_color: BackgroundColor(Color::rgba(0.1, 0.1, 0.1, 1.0)),
                            style: Style {
                                padding: BUTTON_MARGIN,
                                margin: BUTTON_MARGIN,
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        Button::Connect,
                    ))
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
                    .spawn((
                        ButtonBundle {
                            background_color: BackgroundColor(Color::rgba(0.1, 0.1, 0.1, 1.0)),
                            style: Style {
                                padding: BUTTON_MARGIN,
                                margin: BUTTON_MARGIN,
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        Button::Quit,
                    ))
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

fn destroy_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

fn handle_buttons(
    mut commands: Commands,
    mut game_state: ResMut<State<GameState>>,
    mut exit: EventWriter<AppExit>,
    query: Query<(&Button, &Interaction), Changed<Interaction>>,
) {
    for (btn, interaction) in query.iter() {
        if *interaction != Interaction::Clicked {
            continue;
        }

        match *btn {
            Button::Connect => {
                let client = create_renet_client();
                commands.insert_resource(client);
                let _ = game_state.set(GameState::Connecting);
            }
            Button::Quit => exit.send_default(),
        }
    }
}
