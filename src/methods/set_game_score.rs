use crate::types::{Game, User};
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct SetGameScore {
    /// User identifier
    pub user_id: i32,
    /// New score, must be non-negative
    pub score: i32,
    /// Pass True, if the high score is allowed to decrease. This can be useful when fixing mistakes or banning cheaters
    pub force: Option<bool>,
    /// Pass True, if the game message should not be automatically edited to include the current scoreboard
    pub disable_edit_message: Option<bool>,
    /// Required if inline_message_id is not specified. Unique identifier for the target chat
    pub chat_id: Option<i32>,
    /// Required if inline_message_id is not specified. Identifier of the sent message
    pub message_id: Option<i32>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
}

impl SetGameScore {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "SetGameScore")
    }
}
