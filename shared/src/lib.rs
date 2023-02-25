use serde::{Deserialize, Serialize};
use bevy::prelude::*;

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
    },
    PlayerLeft {
        id: u64,
    },

    PlayerTransformUpdate {
        id: u64,
        x: f32,
        y: f32,
        rotation: f32,
    },

    ChatMessage {
        author: u64,
        content: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    PlayerTransformUpdate { x: f32, y: f32, rotation: f32 },
    Shoot { position: Vec2, direction: f32 },
    ChatMessage(String),
}
