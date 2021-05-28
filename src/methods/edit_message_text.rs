use crate::types::{InlineKeyboardMarkup, Message, MessageEntity};
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct EditMessageText {
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: Option<i32>,
    /// Required if inline_message_id is not specified. Identifier of the message to edit
    pub message_id: Option<i32>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
    /// New text of the message, 1-4096 characters after entities parsing
    pub text: String,
    /// Mode for parsing entities in the message text. See formatting options for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in message text, which can be specified instead of parse_mode
    pub entities: Option<Vec<MessageEntity>>,
    /// Disables link previews for links in this message
    pub disable_web_page_preview: Option<bool>,
    /// A JSON-serialized object for an inline keyboard.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageText {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "EditMessageText")
    }
}
