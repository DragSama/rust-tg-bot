use crate::types::{InlineKeyboardMarkup, Message, MessageEntity, User};
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct CopyMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i32,
    /// Unique identifier for the chat where the original message was sent (or channel username in the format @channelusername)
    pub from_chat_id: i32,
    /// Message identifier in the chat specified in from_chat_id
    pub message_id: i32,
    /// New caption for media, 0-1024 characters after entities parsing. If not specified, the original caption is kept
    pub caption: Option<String>,
    /// Mode for parsing entities in the new caption. See formatting options for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the new caption, which can be specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i32>,
    /// Pass True, if the message should be sent even if the specified replied-to message is not found
    pub allow_sending_without_reply: Option<bool>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl CopyMessage {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "CopyMessage")
    }
}
