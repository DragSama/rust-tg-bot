use crate::types::{InlineKeyboardMarkup, Location, Message};
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct StopMessageLiveLocation {
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: Option<i32>,
    /// Required if inline_message_id is not specified. Identifier of the message with live location to stop
    pub message_id: Option<i32>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
    /// A JSON-serialized object for a new inline keyboard.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl StopMessageLiveLocation {
    pub fn get_data(&self) -> (String, &str) {
        (
            serde_json::to_string(&self).unwrap(),
            "StopMessageLiveLocation",
        )
    }
}
