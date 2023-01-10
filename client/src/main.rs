use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Test".to_owned(),
                ..Default::default()
            },
            ..Default::default()
        }))
        .run();
}
