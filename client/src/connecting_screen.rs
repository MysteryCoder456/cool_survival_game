use bevy::prelude::*;
use bevy_renet::*;

use crate::{GameState, UIAssets};

#[derive(Component)]
struct ConnectingScreen;

#[derive(Component)]
struct DotAnimator {
    dots: usize,
    timer: Timer,
}

impl Default for DotAnimator {
    fn default() -> Self {
        Self {
            dots: 0,
            timer: Timer::from_seconds(0.8, TimerMode::Repeating),
        }
    }
}

pub struct ConnectingScreenPlugin;

impl Plugin for ConnectingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Connecting).with_system(setup_connecting_screen),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Connecting).with_system(destroy_connecting_screen),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Connecting)
                .with_system(dot_animation_system)
                .with_system(handle_client_connection_state),
        );
    }
}

fn setup_connecting_screen(mut commands: Commands, game_assets: Res<UIAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    align_self: AlignSelf::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    padding: UiRect::all(Val::Percent(1.)),
                    ..Default::default()
                },
                ..Default::default()
            },
            ConnectingScreen,
        ))
        .with_children(|node| {
            node.spawn((
                TextBundle::from_section(
                    "Connecting...".to_owned(),
                    TextStyle {
                        font_size: 40.0,
                        color: Color::WHITE,
                        font: game_assets.font.clone(),
                    },
                ),
                DotAnimator::default(),
            ));
        });
}

fn destroy_connecting_screen(mut commands: Commands, query: Query<Entity, With<ConnectingScreen>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

fn dot_animation_system(time: Res<Time>, mut query: Query<(&mut DotAnimator, &mut Text)>) {
    for (mut animator, mut text) in query.iter_mut() {
        animator.timer.tick(time.delta());

        if animator.timer.finished() {
            if animator.dots > 3 {
                animator.dots = 0;
            }

            let new_value = format!("Connecting{}", ".".repeat(animator.dots));
            text.sections[0].value = new_value;
            animator.dots += 1;
        }
    }
}

fn handle_client_connection_state(
    mut game_state: ResMut<State<GameState>>,
    client: Res<renet::RenetClient>,
) {
    if client.is_changed() && client.is_connected() {
        let _ = game_state.set(GameState::Game);
    };
}
