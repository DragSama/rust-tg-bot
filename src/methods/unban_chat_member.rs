use crate::types::{Chat, ChatMember};
use serde::Serialize;
use serde_json;


#[derive(Debug, Serialize)]
pub struct UnbanChatMember{
     /// Unique identifier for the target group or username of the target supergroup or channel (in the format @username)
    pub chat_id: i32,
     /// Unique identifier of the target user
    pub user_id: i32,
     /// Do nothing if the user is not banned
    pub only_if_banned: Option<bool>
}

impl UnbanChatMember {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "UnbanChatMember")
}}