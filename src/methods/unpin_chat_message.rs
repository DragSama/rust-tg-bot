use crate::types::{Chat, Message};
use serde::Serialize;
use serde_json;


#[derive(Debug, Serialize)]
pub struct UnpinChatMessage{
     /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i32,
     /// Identifier of a message to unpin. If not specified, the most recent pinned message (by sending date) will be unpinned.
    pub message_id: Option<i32>
}

impl UnpinChatMessage {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "UnpinChatMessage")
}}