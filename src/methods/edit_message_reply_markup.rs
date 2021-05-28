use crate::types::{InlineKeyboardMarkup, Message};
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct EditMessageReplyMarkup {
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: Option<i32>,
    /// Required if inline_message_id is not specified. Identifier of the message to edit
    pub message_id: Option<i32>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
    /// A JSON-serialized object for an inline keyboard.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageReplyMarkup {
    pub fn get_data(&self) -> (String, &str) {
        (
            serde_json::to_string(&self).unwrap(),
            "EditMessageReplyMarkup",
        )
    }
}
