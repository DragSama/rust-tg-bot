use crate::types::{Chat, ChatPhoto};
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct DeleteChatPhoto {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i32,
}

impl DeleteChatPhoto {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "DeleteChatPhoto")
    }
}
