use serde::{Deserialize, Serialize};

pub const PROTOCOL_ID: u64 = 1;

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    PlayerJoined { id: u64, username: String },
    PlayerLeft { id: u64 },
    ServerMessage(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    ChatMessage(String),
}
