use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use super::Orcs;
use crate::GameState;
use shared::ServerMessage;

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

#[derive(Resource)]
struct OrcAssets {
    idle: Handle<TextureAtlas>,
}

pub struct OrcPlugin;

impl Plugin for OrcPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<events::SpawnOrc>()
            .add_startup_system(setup_orc)
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(spawn_orc_system)
                    .with_system(orc_transform_update_system),
            );
    }
}

fn setup_orc(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let idle_texture: Handle<Image> = asset_server.load("textures/orc/idle.png");
    let idle_atlas = TextureAtlas::from_grid(idle_texture, Vec2::new(64.0, 64.0), 1, 1, None, None);
    let idle = texture_atlases.add(idle_atlas);

    let orc_assets = OrcAssets { idle };

    commands.insert_resource(orc_assets);
}

fn spawn_orc_system(
    mut commands: Commands,
    mut events: EventReader<events::SpawnOrc>,
    mut orcs: ResMut<Orcs>,
    orc_assets: Res<OrcAssets>,
) {
    for event in events.iter() {
        let entity = commands
            .spawn((
                SpriteSheetBundle {
                    texture_atlas: orc_assets.idle.clone(),
                    transform: Transform {
                        translation: event.position.extend(0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Orc(event.id),
            ))
            .id();

        // Add entity to orcs roster
        orcs.0.insert(event.id, entity);
    }
}

fn orc_transform_update_system(
    mut events: EventReader<ServerMessage>,
    mut query: Query<&mut Transform, With<Orc>>,
    orcs: Res<Orcs>,
) {
    for event in events.iter() {
        if let ServerMessage::OrcTransformUpdate {
            id,
            position,
            rotation,
        } = event
        {
            // Get orc entity from orc id
            let entity = orcs.0.get(id);
            if entity.is_none() {
                continue;
            }
            let entity = entity.unwrap();

            if let Ok(mut orc_tf) = query.get_mut(*entity) {
                // Mutate entity's transform
                // FIXME: understand quaternions
                orc_tf.translation = position.extend(0.0);
                orc_tf.rotation = Quat::from_rotation_z(*rotation);
                dbg!(rotation, rotation * 3.141592);
            }
        }
    }
}
