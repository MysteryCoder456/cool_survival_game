use std::{
    error::Error,
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy::{app::AppExit, prelude::*};
use bevy_renet::{renet::NETCODE_USER_DATA_BYTES, *};

use crate::{GameState, UIAssets};
use shared::*;

const BUTTON_MARGIN: UiRect = UiRect {
    top: Val::Px(10.0),
    bottom: Val::Px(10.0),
    left: Val::Px(10.0),
    right: Val::Px(10.0),
};

const BUTTON_MIN_SIZE: Size = Size {
    width: Val::Auto,
    height: Val::Px(42.0),
};

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
enum Button {
    Connect,
    Quit,
}

#[derive(Component)]
struct UsernameText;

#[derive(Resource)]
pub struct MyUsername(pub String);

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MyUsername(String::new()))
            .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup_main_menu))
            .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(destroy_main_menu))
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu)
                    .with_system(handle_username_input)
                    .with_system(handle_buttons),
            );
    }
}

fn create_renet_client(user_data: UserData) -> Result<renet::RenetClient, Box<dyn Error>> {
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;

    // TODO: Implement entering a custom server address
    let server_addr: SocketAddr = "127.0.0.1:5678".parse()?;
    let socket = UdpSocket::bind("127.0.0.1:0")?;

    let config = renet::RenetConnectionConfig::default();

    let user_data_serialized = {
        // Check whether the serialized data would be small enough
        if bincode::serialized_size::<UserData>(&user_data)? as usize > NETCODE_USER_DATA_BYTES {
            return Err(format!(
                "User data is too large (>{} bytes)",
                NETCODE_USER_DATA_BYTES
            )
            .into());
        }

        let user_data = bincode::serialize(&user_data)?;
        let mut data_array: [u8; 256] = [0; NETCODE_USER_DATA_BYTES];

        for i in 0..user_data.len() {
            data_array[i] = user_data[i];
        }

        data_array
    };

    let authentication = renet::ClientAuthentication::Unsecure {
        protocol_id: PROTOCOL_ID,
        client_id: current_time.as_millis() as u64,
        server_addr,
        user_data: Some(user_data_serialized),
    };

    renet::RenetClient::new(current_time, socket, config, authentication).map_err(|e| e.into())
}

fn setup_main_menu(mut commands: Commands, ui_assets: Res<UIAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    padding: UiRect::all(Val::Px(20.0)),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                ..Default::default()
            },
            MainMenu,
        ))
        .with_children(|node| {
            // Top Row
            node.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Px(40.0)),
                    justify_content: JustifyContent::Center,
                    align_content: AlignContent::Center,
                    flex_wrap: FlexWrap::Wrap,
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|row_node| {
                row_node.spawn(TextBundle::from_section(
                    "Cool Survival Game",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::WHITE,
                        font: ui_assets.font.clone(),
                    },
                ));
            });

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
                // Username "Field"
                center_node
                    .spawn(NodeBundle {
                        background_color: BackgroundColor(Color::rgb(0.48, 0.48, 0.48)),
                        style: Style {
                            border: UiRect::all(Val::Px(5.0)),
                            padding: BUTTON_MARGIN,
                            margin: BUTTON_MARGIN,
                            min_size: BUTTON_MIN_SIZE,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|field_node| {
                        field_node.spawn((
                            TextBundle::from_section(
                                "Enter A Username",
                                TextStyle {
                                    font_size: 25.0,
                                    color: Color::WHITE,
                                    font: ui_assets.font.clone(),
                                },
                            ),
                            UsernameText,
                        ));
                    });

                // Connect Button
                center_node
                    .spawn((
                        ButtonBundle {
                            background_color: BackgroundColor(Color::rgb(0.1, 0.1, 0.1)),
                            style: Style {
                                padding: BUTTON_MARGIN,
                                margin: BUTTON_MARGIN,
                                min_size: BUTTON_MIN_SIZE,
                                align_items: AlignItems::Center,
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
                                font: ui_assets.font.clone(),
                            },
                        ));
                    });

                // Quit Button
                center_node
                    .spawn((
                        ButtonBundle {
                            background_color: BackgroundColor(Color::rgb(0.1, 0.1, 0.1)),
                            style: Style {
                                padding: BUTTON_MARGIN,
                                margin: BUTTON_MARGIN,
                                min_size: BUTTON_MIN_SIZE,
                                align_items: AlignItems::Center,
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
                                font: ui_assets.font.clone(),
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

fn handle_username_input(
    mut events: EventReader<ReceivedCharacter>,
    mut query: Query<&mut Text, With<UsernameText>>,
    mut username: ResMut<MyUsername>,
    kb: Res<Input<KeyCode>>,
) {
    for received in events.iter() {
        let ch = received.char;
        if !(ch.is_control() || ch.is_whitespace()) {
            username.0.push(received.char);
        }
    }

    if kb.just_pressed(KeyCode::Back) {
        username.0.pop();
    }

    let mut username_text = query.single_mut();
    let mut section = &mut username_text.sections[0];

    if username.0.is_empty() {
        section.value = "Enter A Username".to_owned();
    } else {
        // Only copy the username if it's changed
        section.value = format!("Username: {}", username.0);
    }
}

fn handle_buttons(
    mut commands: Commands,
    mut game_state: ResMut<State<GameState>>,
    mut exit: EventWriter<AppExit>,
    username: Res<MyUsername>,
    query: Query<(&Button, &Interaction), Changed<Interaction>>,
) {
    for (btn, interaction) in query.iter() {
        if *interaction != Interaction::Clicked {
            continue;
        }

        match *btn {
            Button::Connect => {
                // Only connect if username is not empty
                if username.0.is_empty() {
                    break;
                }

                let client = create_renet_client(UserData {
                    username: username.0.trim().to_owned(),
                });

                match client {
                    Ok(client) => {
                        commands.insert_resource(client);
                        let _ = game_state.set(GameState::Connecting);
                    }
                    Err(e) => {
                        // TODO: Display error message in GUI.
                        eprintln!("{e:?}");
                    }
                };
            }
            Button::Quit => exit.send_default(),
        }
    }
}
