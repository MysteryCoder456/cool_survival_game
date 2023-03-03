use bevy::prelude::*;

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

pub struct OrcPlugin;

impl Plugin for OrcPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<events::SpawnOrc>()
            .add_event::<events::OrcTransformUpdate>();
    }
}
