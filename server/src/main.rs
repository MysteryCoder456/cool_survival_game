use std::{
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy::prelude::*;
use bevy_renet::{renet::ServerEvent, *};

use shared::PROTOCOL_ID;

const MAX_PLAYERS: usize = 10;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(RenetServerPlugin::default())
        .insert_resource(create_renet_server())
        .add_system(handle_client_messages)
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

fn handle_client_messages(mut server: ResMut<renet::RenetServer>) {
    let channel_id = 0;

    for client_id in server.clients_id() {
        while let Some(msg) = server.receive_message(client_id, channel_id) {
            let msg_str = msg.iter().map(|c| char::from(*c)).collect::<String>();
            println!("{} sent a message: {}", client_id, msg_str);
        }
    }
}

fn handle_server_events(mut events: EventReader<ServerEvent>) {
    for event in events.iter() {
        match event {
            ServerEvent::ClientConnected(id, _) => println!("{} has joined the game", id),
            ServerEvent::ClientDisconnected(id) => println!("{} has left the game", id),
        }
    }
}
