use bevy::prelude::*;

use crate::{orc::events::*, Broadcast, PlayerInfo, Players, CM};
use shared::*;

pub mod events {
    use bevy::prelude::Vec2;

    pub struct SpawnPlayer {
        pub id: u64,
        pub position: Vec2,
        pub username: String,
    }

    pub struct DespawnPlayer {
        pub id: u64,
    }
}

#[derive(Component)]
struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<events::SpawnPlayer>()
            .add_event::<events::DespawnPlayer>()
            .add_system(spawn_player_system)
            .add_system(despawn_player_system)
            .add_system(player_transform_update_system)
            .add_system(player_shoot_system);
    }
}

fn spawn_player_system(
    mut commands: Commands,
    mut events: EventReader<events::SpawnPlayer>,
    mut players: ResMut<Players>,
) {
    for event in events.iter() {
        let entity = commands
            .spawn((
                TransformBundle {
                    local: Transform {
                        translation: event.position.extend(0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Player,
            ))
            .id();

        players.0.insert(
            event.id,
            PlayerInfo {
                entity,
                username: event.username.clone(),
            },
        );
    }
}

fn despawn_player_system(
    mut commands: Commands,
    mut events: EventReader<events::DespawnPlayer>,
    mut players: ResMut<Players>,
) {
    for event in events.iter() {
        if let Some(player_info) = players.0.remove(&event.id) {
            commands.entity(player_info.entity).despawn();
        }
    }
}

fn player_transform_update_system(
    mut client_msg_events: EventReader<CM>,
    mut server_broadcast_events: EventWriter<Broadcast>,
    mut query: Query<&mut Transform, With<Player>>,
    players: Res<Players>,
) {
    for client_msg in client_msg_events.iter() {
        if let (player_id, ClientMessage::PlayerTransformUpdate { position, rotation }) = client_msg
        {
            let player_info = players.0.get(player_id);
            if player_info.is_none() {
                continue;
            }

            let player_info = player_info.unwrap();
            let mut player_tf = query.get_mut(player_info.entity).unwrap();

            // Update server player info
            player_tf.translation = position.extend(0.0);
            player_tf.rotation = Quat::from_rotation_z(*rotation);

            server_broadcast_events.send(Broadcast {
                message: ServerMessage::PlayerTransformUpdate {
                    id: *player_id,
                    position: *position,
                    rotation: *rotation,
                },
                except: Some(*player_id),
            });
        }
    }
}

fn player_shoot_system(
    mut client_msg_events: EventReader<CM>,
    mut server_broadcast_events: EventWriter<Broadcast>,
    mut spawn_orc_events: EventWriter<SpawnOrc>,
    query: Query<&Transform, With<Player>>,
    players: Res<Players>,
) {
    // TODO: Shoot cooldown

    for (player_id, client_msg) in client_msg_events.iter() {
        if let ClientMessage::Shoot { direction } = client_msg {
            let player_info = players.0.get(player_id);
            if player_info.is_none() {
                continue;
            }

            let player_info = player_info.unwrap();
            let player_tf = query.get(player_info.entity).unwrap();
            let orc_position = player_tf.translation.truncate();
            let direction = *direction;

            let orc_id = rand::random::<u64>();

            // Spawn orc in server world
            spawn_orc_events.send(SpawnOrc {
                id: orc_id,
                position: orc_position,
                direction,
            });

            // Broadcast spawn orc message
            server_broadcast_events.send(Broadcast {
                message: ServerMessage::SpawnOrc {
                    id: orc_id,
                    position: orc_position,
                    direction,
                },
                except: None,
            });
        }
    }
}
