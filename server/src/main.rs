use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy::prelude::*;
use bevy_renet::{renet::ServerEvent, *};

use shared::*;

const MAX_PLAYERS: usize = 10;

struct PlayerInfo {
    username: String,
}

#[derive(Resource, Default)]
struct Players(HashMap<u64, PlayerInfo>);

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(RenetServerPlugin::default())
        .insert_resource(create_renet_server())
        .insert_resource(Players::default())
        .add_event::<ServerMessage>()
        .add_event::<(u64, ServerMessage)>()
        .add_event::<(u64, ClientMessage)>()
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
        MAX_PLAYERS,
        PROTOCOL_ID,
        server_addr,
        renet::ServerAuthentication::Unsecure,
    );

    let connection_config = renet::RenetConnectionConfig::default();
    let socket = UdpSocket::bind(server_addr).unwrap();

    renet::RenetServer::new(current_time, server_config, connection_config, socket).unwrap()
}

fn handle_incoming_messages(
    mut server: ResMut<renet::RenetServer>,
    mut events: EventWriter<(u64, ClientMessage)>,
) {
    let channel_id = 0;

    for client_id in server.clients_id() {
        while let Some(serialized_msg) = server.receive_message(client_id, channel_id) {
            match bincode::deserialize(&serialized_msg) {
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
    mut events: EventReader<ServerMessage>,
) {
    let channel_id = 0;

    for server_msg in events.iter() {
        match bincode::serialize(server_msg) {
            Ok(serialized_msg) => server.broadcast_message(channel_id, serialized_msg),
            Err(error) => eprintln!(
                "An error occured while serializing {:?}:\n{}",
                server_msg, error
            ),
        }
    }
}

/// Handle server messages which have to be sent to only one specific client
fn handle_outgoing_messages(
    mut server: ResMut<renet::RenetServer>,
    mut events: EventReader<(u64, ServerMessage)>,
) {
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
    mut server_broadcast_events: EventWriter<ServerMessage>,
    mut server_msg_events: EventWriter<(u64, ServerMessage)>,
    mut players: ResMut<Players>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(new_id, _) => {
                println!("{} has joined the game", new_id);
                let username = format!("John Doe {}", new_id); // TODO: Fetch username from client

                // Inform other players about new player
                server_broadcast_events.send(ServerMessage::PlayerJoined {
                    id: *new_id,
                    username: username.clone(),
                    // TODO: Add spawn position
                });

                // Inform new player about existing players
                players.0.iter().for_each(|(player_id, player_info)| {
                    server_msg_events.send((
                        *new_id,
                        ServerMessage::PlayerJoined {
                            id: *player_id,
                            username: player_info.username.clone(),
                        },
                    ));
                });

                // Add new player info to players list
                players.0.insert(
                    *new_id,
                    PlayerInfo {
                        username: username.clone(),
                    },
                );
            }
            ServerEvent::ClientDisconnected(id) => {
                println!("{} has left the game", id);
                server_broadcast_events.send(ServerMessage::PlayerLeft { id: *id });
                players.0.remove(id);
            }
        }
    }
}
