use std::f32::consts::FRAC_PI_2;

use bevy::{prelude::*, time::FixedTimestep};

use shared::*;

use super::{CursorWorldPosition, PlayerInfo, Players, PHYSICS_TIMESTEP};
use crate::{main_menu::MyUsername, GameState, MainCamera};

const PLAYER_SPEED: f32 = 300.0;

#[derive(Component)]
struct Player;

#[derive(Resource)]
struct PlayerAssets {
    idle: Handle<TextureAtlas>,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_player)
            .add_system_set(SystemSet::on_enter(GameState::Game).with_system(spawn_player_system))
            .add_system_set(SystemSet::on_update(GameState::Game).with_system(player_shoot_system))
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(PHYSICS_TIMESTEP))
                    .with_run_criteria(State::on_update(GameState::Game))
                    .with_system(player_movement_system)
                    .with_system(camera_follow_system),
            );
    }
}

fn setup_player(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let idle_texture: Handle<Image> = asset_server.load("textures/player/idle.png");
    let idle_atlas = TextureAtlas::from_grid(idle_texture, Vec2::new(64.0, 64.0), 1, 1, None, None);
    let idle = texture_atlases.add(idle_atlas);

    let player_assets = PlayerAssets { idle };

    commands.insert_resource(player_assets);
}

fn spawn_player_system(
    mut commands: Commands,
    mut players: ResMut<Players>,
    client: Res<bevy_renet::renet::RenetClient>,
    username: Res<MyUsername>,
    player_assets: Res<PlayerAssets>,
) {
    let entity = commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: player_assets.idle.clone(),
                ..Default::default()
            },
            Player,
        ))
        .id();

    players.0.insert(
        client.client_id(),
        PlayerInfo {
            entity,
            username: username.0.clone(),
        },
    );
}

fn player_movement_system(
    time: Res<Time>,
    kb: Res<Input<KeyCode>>,
    cursor_pos: Res<CursorWorldPosition>,
    mut query: Query<&mut Transform, With<Player>>,
    mut events: EventWriter<ClientMessage>,
) {
    if query.is_empty() {
        return;
    }
    let mut transform = query.single_mut();
    let old_transform = transform.clone();

    // Calculate the direction in which the player will move
    let direction = Vec2 {
        x: (kb.pressed(KeyCode::D) as i32 - kb.pressed(KeyCode::A) as i32) as f32,
        y: (kb.pressed(KeyCode::W) as i32 - kb.pressed(KeyCode::S) as i32) as f32,
    }
    .normalize_or_zero();
    let displacement = direction.extend(0.0) * PLAYER_SPEED * time.delta_seconds();

    // Translate the player
    transform.translation.x += displacement.x;
    transform.translation.y += displacement.y;

    // Rotate the player
    let diff = cursor_pos.0 - transform.translation.truncate();
    let angle = diff.y.atan2(diff.x);
    transform.rotation = Quat::from_rotation_z(angle - FRAC_PI_2);

    // Send transform update to server if transform has changed
    if *transform != old_transform {
        events.send(ClientMessage::PlayerTransformUpdate {
            position: transform.translation.truncate(),
            rotation: angle,
        });
    }
}

fn camera_follow_system(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<(&mut Transform, &MainCamera), Without<Player>>,
) {
    if player_query.is_empty() || camera_query.is_empty() {
        return;
    }
    let player_tf = player_query.single();
    let (mut camera_tf, camera) = camera_query.single_mut();

    let from = camera_tf.translation;
    let to = player_tf.translation;

    let direction = (to - from).normalize_or_zero();
    let speed_adjustment = from.distance(to) / camera.follow_distance;
    let camera_velocity = direction * speed_adjustment * camera.speed * time.delta_seconds();

    camera_tf.translation += camera_velocity;
}

fn player_shoot_system(
    mut events: EventWriter<ClientMessage>,
    mouse: Res<Input<MouseButton>>,
    cursor_pos: Res<CursorWorldPosition>,
    query: Query<&Transform, With<Player>>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let player_tf = query.single();
    let diff = cursor_pos.0 - player_tf.translation.truncate();
    let direction = diff.y.atan2(diff.x);

    events.send(ClientMessage::Shoot { direction });
}
