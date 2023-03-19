use bevy::prelude::*;

use crate::GameState;

pub mod events {
    use bevy::prelude::Vec2;

    pub struct SpawnOrc {
        pub id: u64,
        pub position: Vec2,
        pub direction: f32,
    }

    pub struct OrcTransformUpdate {
        pub id: u64,
        pub position: Vec2,
        pub direction: f32,
    }
}

#[derive(Resource)]
struct OrcAssets {
    idle: Handle<TextureAtlas>,
}

pub struct OrcPlugin;

impl Plugin for OrcPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<events::SpawnOrc>()
            .add_event::<events::OrcTransformUpdate>()
            .add_startup_system(setup_orc)
            .add_system_set(SystemSet::on_update(GameState::Game).with_system(spawn_orc_system));
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

fn spawn_orc_system(mut commands: Commands, mut events: EventReader<events::SpawnOrc>, orc_assets: Res<OrcAssets>) {
    for event in events.iter() {
        // TODO:
    }
}
