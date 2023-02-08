use std::{
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy::prelude::*;
use bevy_renet::{renet::ServerEvent, *};

use shared::*;

const MAX_PLAYERS: usize = 10;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(RenetServerPlugin::default())
        .insert_resource(create_renet_server())
        .add_event::<ServerMessage>()
        .add_event::<(u64, ClientMessage)>()
        .add_system(handle_incoming_messages)
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

fn handle_outgoing_messages(
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

fn handle_server_events(
    mut server_events: EventReader<ServerEvent>,
    mut server_msg_events: EventWriter<ServerMessage>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(new_id, _) => {
                println!("{} has joined the game", new_id);

                server_msg_events.send(ServerMessage::PlayerJoined {
                    id: *new_id,
                    username: format!("John Doe {}", new_id), // TODO: Fetch username
                });
            }
            ServerEvent::ClientDisconnected(id) => {
                println!("{} has left the game", id);
                server_msg_events.send(ServerMessage::PlayerLeft { id: *id })
            }
        }
    }
}
