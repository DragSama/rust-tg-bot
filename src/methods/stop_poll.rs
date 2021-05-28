use crate::types::{InlineKeyboardMarkup, Poll};
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct StopPoll {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i32,
    /// Identifier of the original message with the poll
    pub message_id: i32,
    /// A JSON-serialized object for a new message inline keyboard.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl StopPoll {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "StopPoll")
    }
}
