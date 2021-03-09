use crate::types::{Chat, ChatPermissions};
use serde::Serialize;
use serde_json;


#[derive(Debug, Serialize)]
pub struct SetChatPermissions{
     /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: i32,
     /// New default chat permissions
    pub permissions: ChatPermissions
}

impl SetChatPermissions {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "SetChatPermissions")
}}