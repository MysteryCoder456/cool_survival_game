use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub const PROTOCOL_ID: u64 = 1;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    PlayerJoined {
        id: u64,
        username: String,
        position: Vec2,
    },
    PlayerLeft {
        id: u64,
    },

    PlayerTransformUpdate {
        id: u64,
        position: Vec2,
        rotation: f32,
    },

    SpawnOrc {
        position: Vec2,
        direction: f32,
    },

    ChatMessage {
        author: u64,
        content: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    PlayerTransformUpdate { position: Vec2, rotation: f32 },
    Shoot { direction: f32 },
    ChatMessage(String),
}
