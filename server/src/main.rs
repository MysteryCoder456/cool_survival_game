use bevy::prelude::*;
use bevy_renet::*;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(RenetServerPlugin::default())
        .run();
}
