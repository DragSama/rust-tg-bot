use crate::types::{Game, InlineKeyboardMarkup, User};
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct SendGame {
    /// Unique identifier for the target chat
    pub chat_id: i32,
    /// Short name of the game, serves as the unique identifier for the game. Set up your games via Botfather.
    pub game_short_name: String,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i32>,
    /// Pass True, if the message should be sent even if the specified replied-to message is not found
    pub allow_sending_without_reply: Option<bool>,
    /// A JSON-serialized object for an inline keyboard. If empty, one 'Play game_title' button will be shown. If not empty, the first button must launch the game.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl SendGame {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "SendGame")
    }
}
