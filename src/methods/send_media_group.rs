use crate::types::{Audio, InputMedia, InputMediaAudio, User};
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct SendMediaGroup {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i32,
    /// A JSON-serialized array describing messages to be sent, must include 2-10 items
    pub media: Vec<InputMediaAudio>,
    /// Sends messages silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// If the messages are a reply, ID of the original message
    pub reply_to_message_id: Option<i32>,
    /// Pass True, if the message should be sent even if the specified replied-to message is not found
    pub allow_sending_without_reply: Option<bool>,
}

impl SendMediaGroup {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "SendMediaGroup")
    }
}
