use bevy::prelude::*;
use bevy_renet::*;

use shared::*;

mod player;

use crate::GameState;
use player::PlayerPlugin;

pub struct MainGamePlugin;

impl Plugin for MainGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin)
            .add_event::<ServerMessage>()
            .add_event::<ClientMessage>()
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(handle_incoming_messages)
                    .with_system(handle_outgoing_messages),
            );
    }
}

// TODO: Build game world

fn handle_incoming_messages(
    mut client: ResMut<renet::RenetClient>,
    mut events: EventWriter<ServerMessage>,
) {
    let channel_id = 0;

    while let Some(serialized_msg) = client.receive_message(channel_id) {
        match bincode::deserialize(&serialized_msg) {
            Ok(server_msg) => events.send(server_msg),
            Err(error) => eprintln!(
                "An error occured while deserializing server message:\n{}",
                error
            ),
        }
    }
}

fn handle_outgoing_messages(
    mut client: ResMut<renet::RenetClient>,
    mut events: EventReader<ClientMessage>,
) {
    let channel_id = 0;

    for client_msg in events.iter() {
        match bincode::serialize(client_msg) {
            Ok(serialized_msg) => client.send_message(channel_id, serialized_msg),
            Err(error) => eprintln!(
                "An error occured while serializing {:?}:\n{}",
                client_msg, error
            ),
        }
    }
}
