use crate::types::{InlineKeyboardMarkup, Message, MessageEntity, User};
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct SendMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// Text of the message to be sent, 1-4096 characters after entities parsing
    pub text: String,
    /// Mode for parsing entities in the message text. See formatting options for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in message text, which can be specified instead of parse_mode
    pub entities: Option<Vec<MessageEntity>>,
    /// Disables link previews for links in this message
    pub disable_web_page_preview: Option<bool>,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i32>,
    /// Pass True, if the message should be sent even if the specified replied-to message is not found
    pub allow_sending_without_reply: Option<bool>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl SendMessage {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "SendMessage")
    }
}
