use crate::types::{Chat, Message};
use serde::Serialize;
use serde_json;


#[derive(Debug, Serialize)]
pub struct PinChatMessage{
     /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i32,
     /// Identifier of a message to pin
    pub message_id: i32,
     /// Pass True, if it is not necessary to send a notification to all chat members about the new pinned message. Notifications are always disabled in channels and private chats.
    pub disable_notification: Option<bool>
}

impl PinChatMessage {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "PinChatMessage")
}}