use shared::PROTOCOL_ID;
use std::{net::UdpSocket, time::SystemTime};

use bevy::prelude::*;
use bevy_renet::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Test".to_owned(),
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(RenetClientPlugin::default())
        .insert_resource(create_renet_config())
        .run();
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
