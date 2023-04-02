use bevy::prelude::*;

use crate::{components::Velocity, Broadcast};
use shared::ServerMessage;

pub mod events {
    use bevy::prelude::Vec2;

    pub struct SpawnOrc {
        pub id: u64,
        pub position: Vec2,
        pub direction: f32,
    }
}

const ORC_SPEED: f32 = 6000.0;

#[derive(Component)]
struct Orc(u64);

pub struct OrcPlugin;

impl Plugin for OrcPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<events::SpawnOrc>()
            .add_system(spawn_orc_system)
            .add_system(orc_transform_update_system);
    }
}

fn spawn_orc_system(mut commands: Commands, mut events: EventReader<events::SpawnOrc>) {
    for event in events.iter() {
        commands.spawn((
            TransformBundle {
                local: Transform {
                    translation: event.position.extend(0.0),
                    rotation: Quat::from_rotation_z(event.direction),
                    ..Default::default()
                },
                ..Default::default()
            },
            Orc(event.id),
            Velocity(Vec2::from_angle(event.direction) * ORC_SPEED),
        ));
    }
}

fn orc_transform_update_system(
    mut events: EventWriter<Broadcast>,
    query: Query<(&Transform, &Orc), Changed<Transform>>,
) {
    let broadcasts = query.iter().map(|(orc_tf, orc)| Broadcast {
        message: ServerMessage::OrcTransformUpdate {
            id: orc.0,
            position: orc_tf.translation.truncate(),
            rotation: orc_tf.rotation.z,
        },
        except: None,
    });

    // Send all transform update broadcasts together
    events.send_batch(broadcasts);
}
