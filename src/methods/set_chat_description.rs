use crate::types::Chat;
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct SetChatDescription {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i32,
    /// New chat description, 0-255 characters
    pub description: Option<String>,
}

impl SetChatDescription {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "SetChatDescription")
    }
}
