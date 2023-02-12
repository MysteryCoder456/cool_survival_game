use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use shared::*;

use super::{PlayerInfo, Players};
use crate::GameState;

pub mod events {
    use bevy::prelude::*;

    pub struct SpawnSlavePlayer {
        pub id: u64,
        pub username: String,
        pub position: Vec2,
    }
}

#[derive(Component)]
struct SlavePlayer(u64);

#[derive(Resource)]
struct SlavePlayerAssets {
    idle: Handle<TextureAtlas>,
}

pub struct SlavePlayerPlugin;

impl Plugin for SlavePlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<events::SpawnSlavePlayer>()
            .add_startup_system(setup_slave_player)
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(spawn_slave_player_system)
                    .with_system(transform_slave_player_system),
            );
    }
}

fn setup_slave_player(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let idle_texture: Handle<Image> = asset_server.load("textures/slave_player/idle.png");
    let idle_atlas = TextureAtlas::from_grid(idle_texture, Vec2::new(64.0, 64.0), 1, 1, None, None);
    let idle = texture_atlases.add(idle_atlas);

    let player_assets = SlavePlayerAssets { idle };

    commands.insert_resource(player_assets);
}

fn spawn_slave_player_system(
    mut commands: Commands,
    mut events: EventReader<events::SpawnSlavePlayer>,
    mut players: ResMut<Players>,
    slave_player_assets: Res<SlavePlayerAssets>,
) {
    for event in events.iter() {
        let entity = commands
            .spawn((
                SpriteSheetBundle {
                    texture_atlas: slave_player_assets.idle.clone(),
                    transform: Transform {
                        translation: event.position.extend(0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                SlavePlayer(event.id),
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

fn transform_slave_player_system(
    mut events: EventReader<ServerMessage>,
    mut query: Query<&mut Transform, With<SlavePlayer>>,
    players: Res<Players>,
) {
    for server_msg in events.iter() {
        if let ServerMessage::PlayerTransformUpdate { id, x, y, rotation } = server_msg {
            if let Some(info) = players.0.get(id) {
                let mut transform = query.get_mut(info.entity).unwrap();

                transform.translation = Vec3::new(*x, *y, 0.0);
                transform.rotation = Quat::from_rotation_z(*rotation - FRAC_PI_2);
            }
        }
    }
}
