use crate::types::{Chat, ChatMember};
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct GetChatMember {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: i32,
    /// Unique identifier of the target user
    pub user_id: i32,
}

impl GetChatMember {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "GetChatMember")
    }
}
