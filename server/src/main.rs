use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy::prelude::*;
use bevy_renet::{renet::ServerEvent, *};

use player::{events::*, PlayerPlugin};
use shared::*;

mod orc;
mod player;

const MAX_CLIENTS: usize = 10;
const PLAYER_SPAWN: Vec2 = Vec2::ZERO;

// u64 value corresponds to the recipient/sender id
type SM = (u64, ServerMessage);
type CM = (u64, ClientMessage);

struct Broadcast {
    message: ServerMessage,
    except: Option<u64>,
}

struct PlayerInfo {
    entity: Entity,
    username: String,
}

#[derive(Resource, Default)]
struct Players(HashMap<u64, PlayerInfo>);

fn main() {
    App::new()
        .add_plugins(MinimalPlugins.set(CorePlugin {
            task_pool_options: TaskPoolOptions::with_num_threads(1),
        }))
        .add_plugin(RenetServerPlugin::default())
        .add_plugin(PlayerPlugin)
        .insert_resource(create_renet_server())
        .insert_resource(Players::default())
        .add_event::<Broadcast>()
        .add_event::<SM>()
        .add_event::<CM>()
        .add_system(handle_incoming_messages)
        .add_system(handle_outgoing_broadcasts)
        .add_system(handle_outgoing_messages)
        .add_system(handle_server_events)
        .run();
}

fn create_renet_server() -> renet::RenetServer {
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let server_addr: SocketAddr = "127.0.0.1:5678".parse().unwrap();
    let server_config = renet::ServerConfig::new(
        MAX_CLIENTS,
        PROTOCOL_ID,
        server_addr,
        renet::ServerAuthentication::Unsecure,
    );

    let connection_config = renet::RenetConnectionConfig::default();
    let socket = UdpSocket::bind(server_addr).unwrap();

    renet::RenetServer::new(current_time, server_config, connection_config, socket).unwrap()
}

fn handle_incoming_messages(mut server: ResMut<renet::RenetServer>, mut events: EventWriter<CM>) {
    let channel_id = 0;

    for client_id in server.clients_id() {
        while let Some(serialized_msg) = server.receive_message(client_id, channel_id) {
            match bincode::deserialize::<ClientMessage>(&serialized_msg) {
                Ok(client_msg) => events.send((client_id, client_msg)),
                Err(error) => eprintln!(
                    "An error occured while deserializing client message:\n{}",
                    error
                ),
            }
        }
    }
}

/// Handle server messages which have to be sent to all clients
fn handle_outgoing_broadcasts(
    mut server: ResMut<renet::RenetServer>,
    mut events: EventReader<Broadcast>,
) {
    let channel_id = 0;

    for Broadcast { message, except } in events.iter() {
        match bincode::serialize(message) {
            Ok(serialized_msg) => {
                if let Some(except) = except {
                    server.broadcast_message_except(*except, channel_id, serialized_msg);
                } else {
                    server.broadcast_message(channel_id, serialized_msg);
                }
            }
            Err(error) => eprintln!(
                "An error occured while serializing {:?}:\n{}",
                message, error
            ),
        }
    }
}

/// Handle server messages which have to be sent to only one specific client
fn handle_outgoing_messages(mut server: ResMut<renet::RenetServer>, mut events: EventReader<SM>) {
    let channel_id = 0;

    for (recipient_id, server_msg) in events.iter() {
        match bincode::serialize(server_msg) {
            Ok(serialized_msg) => server.send_message(*recipient_id, channel_id, serialized_msg),
            Err(error) => eprintln!(
                "An error occured while serializing {:?}:\n{}",
                server_msg, error
            ),
        }
    }
}

fn handle_server_events(
    mut server_events: EventReader<ServerEvent>,
    mut server_broadcast_events: EventWriter<Broadcast>,
    mut server_msg_events: EventWriter<SM>,
    mut player_spawn_events: EventWriter<SpawnPlayer>,
    mut player_despawn_events: EventWriter<DespawnPlayer>,
    players: Res<Players>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(new_id, user_data) => {
                let user_data = bincode::deserialize::<UserData>(&**user_data);
                if user_data.is_err() {
                    continue;
                }

                let user_data = user_data.unwrap();
                let username = user_data.username.trim();

                println!("{} has joined the game as {}", new_id, user_data.username);

                // Inform existing players about new player
                server_broadcast_events.send(Broadcast {
                    message: ServerMessage::PlayerJoined {
                        id: *new_id,
                        username: username.to_owned(),
                        position: PLAYER_SPAWN,
                    },
                    except: Some(*new_id),
                });

                // Inform new player about existing players
                players.0.iter().for_each(|(player_id, player_info)| {
                    server_msg_events.send((
                        *new_id,
                        ServerMessage::PlayerJoined {
                            id: *player_id,
                            username: player_info.username.clone(),
                            position: PLAYER_SPAWN,
                        },
                    ));
                });

                // Spawn the new player in server world
                player_spawn_events.send(SpawnPlayer {
                    id: *new_id,
                    position: PLAYER_SPAWN,
                    username: username.to_owned(),
                })
            }
            ServerEvent::ClientDisconnected(id) => {
                println!("{} has left the game", id);
                player_despawn_events.send(DespawnPlayer { id: *id });

                server_broadcast_events.send(Broadcast {
                    message: ServerMessage::PlayerLeft { id: *id },
                    except: None,
                });
            }
        }
    }
}
