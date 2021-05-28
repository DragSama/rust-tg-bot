use crate::types::{Message, User};
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct ForwardMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i32,
    /// Unique identifier for the chat where the original message was sent (or channel username in the format @channelusername)
    pub from_chat_id: i32,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// Message identifier in the chat specified in from_chat_id
    pub message_id: i32,
}

impl ForwardMessage {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "ForwardMessage")
    }
}
