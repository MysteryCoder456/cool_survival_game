use bevy::prelude::*;

use super::Broadcast;
use shared::{ClientMessage, ServerMessage};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_transform_update_system);
    }
}

fn player_transform_update_system(
    mut client_msg_events: EventReader<(u64, ClientMessage)>,
    mut server_broadcast_events: EventWriter<Broadcast>,
) {
    for client_msg in client_msg_events.iter() {
        if let (id, ClientMessage::PlayerTransformUpdate { position, rotation }) = client_msg {
            server_broadcast_events.send(Broadcast {
                message: ServerMessage::PlayerTransformUpdate {
                    id: *id,
                    position: *position,
                    rotation: *rotation,
                },
                except: Some(*id),
            });
        }
    }
}
