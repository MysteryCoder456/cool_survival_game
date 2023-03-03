use bevy::prelude::*;

pub mod events {
    use bevy::prelude::Vec2;

    pub struct SpawnOrc {
        pub id: u64,
        pub position: Vec2,
        pub direction: f32,
    }
}

#[derive(Component)]
struct Orc(u64);

pub struct OrcPlugin;

impl Plugin for OrcPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<events::SpawnOrc>()
            .add_system(spawn_orc_system);
    }
}

fn spawn_orc_system(mut commands: Commands, mut events: EventReader<events::SpawnOrc>) {
    for event in events.iter() {
        // TODO: Add a velocity component

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
        ));
    }
}
