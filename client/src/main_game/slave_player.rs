use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use shared::*;

use super::{PlayerInfo, Players};
use crate::{GameState, UIAssets};

pub mod events {
    use bevy::prelude::*;

    pub struct SpawnSlavePlayer {
        pub id: u64,
        pub username: String,
        pub position: Vec2,
    }
}

const USERNAME_LABEL_OFFSET: Vec3 = Vec3::new(0.0, 50.0, 0.0);

#[derive(Component)]
struct SlavePlayer {
    id: u64,
    username_entity: Entity,
}

#[derive(Component)]
struct UsernameLabel;

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
    ui_assets: Res<UIAssets>,
) {
    for event in events.iter() {
        // Username Label
        let username_entity = commands
            .spawn((
                Text2dBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            &event.username,
                            TextStyle {
                                font: ui_assets.font.clone(),
                                font_size: 20.0,
                                color: Color::ANTIQUE_WHITE,
                            },
                        )],
                        alignment: TextAlignment::CENTER,
                    },
                    ..Default::default()
                },
                UsernameLabel,
            ))
            .id();

        let player_entity = commands
            .spawn((
                SpriteSheetBundle {
                    texture_atlas: slave_player_assets.idle.clone(),
                    transform: Transform {
                        translation: event.position.extend(0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                SlavePlayer {
                    id: event.id,
                    username_entity,
                },
            ))
            .id();

        players.0.insert(
            event.id,
            PlayerInfo {
                entity: player_entity,
                username: event.username.clone(),
            },
        );
    }
}

fn transform_slave_player_system(
    mut events: EventReader<ServerMessage>,
    mut slave_query: Query<(&mut Transform, &SlavePlayer)>,
    mut username_query: Query<&mut Transform, (With<UsernameLabel>, Without<SlavePlayer>)>,
    players: Res<Players>,
) {
    for server_msg in events.iter() {
        if let ServerMessage::PlayerTransformUpdate { id, x, y, rotation } = server_msg {
            if let Some(info) = players.0.get(id) {
                if let Ok((mut player_tf, player)) = slave_query.get_mut(info.entity) {
                    let angle = *rotation - FRAC_PI_2;

                    player_tf.translation.x = *x;
                    player_tf.translation.y = *y;
                    player_tf.rotation = Quat::from_rotation_z(angle);

                    let mut username_tf = username_query.get_mut(player.username_entity).unwrap();
                    username_tf.translation = player_tf.translation + USERNAME_LABEL_OFFSET;
                }
            }
        }
    }
}
