use crate::types::{Game, GameHighScore};
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct GetGameHighScores {
    /// Target user id
    pub user_id: i32,
    /// Required if inline_message_id is not specified. Unique identifier for the target chat
    pub chat_id: Option<i32>,
    /// Required if inline_message_id is not specified. Identifier of the sent message
    pub message_id: Option<i32>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
}

impl GetGameHighScores {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "GetGameHighScores")
    }
}
