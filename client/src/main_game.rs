use std::collections::HashMap;

use bevy::{prelude::*, render::camera::RenderTarget};
use bevy_renet::*;

use crate::GameState;
use shared::*;

mod orc;
mod player;
mod slave_player;

use orc::{events::*, OrcPlugin};
use player::PlayerPlugin;
use slave_player::{events::*, SlavePlayerPlugin};

pub const PHYSICS_TIMESTEP: f64 = 1.0 / 60.0; // 60 FPS

struct PlayerInfo {
    entity: Entity,
    username: String,
}

#[derive(Resource, Default)]
struct Players(HashMap<u64, PlayerInfo>);

#[derive(Resource, Default)]
struct CursorWorldPosition(Vec2);

pub struct MainGamePlugin;

impl Plugin for MainGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin)
            .add_plugin(SlavePlayerPlugin)
            .add_plugin(OrcPlugin)
            .add_event::<ServerMessage>()
            .add_event::<ClientMessage>()
            .insert_resource(Players::default())
            .insert_resource(CursorWorldPosition::default())
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(handle_incoming_messages)
                    .with_system(handle_outgoing_messages)
                    .with_system(handle_entity_spawns)
                    .with_system(handle_entity_despawns)
                    .with_system(cursor_world_position_system),
            );
    }
}

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

fn handle_entity_spawns(
    mut server_msg_events: EventReader<ServerMessage>,
    mut spawn_slave_events: EventWriter<SpawnSlavePlayer>,
    mut spawn_orc_events: EventWriter<SpawnOrc>,
) {
    type S = ServerMessage;

    for server_msg in server_msg_events.iter() {
        match server_msg {
            S::PlayerJoined {
                id,
                username,
                position,
            } => spawn_slave_events.send(SpawnSlavePlayer {
                id: *id,
                username: username.clone(),
                position: *position,
            }),
            S::SpawnOrc {
                id,
                position,
                direction,
            } => spawn_orc_events.send(SpawnOrc {
                id: *id,
                position: *position,
                direction: *direction,
            }),
            _ => {}
        }
    }
}

fn handle_entity_despawns(
    mut server_msg_events: EventReader<ServerMessage>,
    mut despawn_slave_events: EventWriter<DespawnSlavePlayer>,
) {
    type S = ServerMessage;

    for server_msg in server_msg_events.iter() {
        match server_msg {
            S::PlayerLeft { id } => despawn_slave_events.send(DespawnSlavePlayer { id: *id }),
            _ => {}
        }
    }
}

fn cursor_world_position_system(
    windows: Res<Windows>,
    query: Query<(&Camera, &GlobalTransform)>,
    mut cursor_world_position: ResMut<CursorWorldPosition>,
) {
    let (camera, camera_transform) = query.single();

    let window = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    if let Some(screen_pos) = window.cursor_position() {
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);

        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        cursor_world_position.0 = world_pos.truncate();
    }
}
