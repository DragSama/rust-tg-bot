use crate::types::{Chat, ChatMember, ChatPermissions};
use serde::Serialize;
use serde_json;


#[derive(Debug, Serialize)]
pub struct RestrictChatMember{
     /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: i32,
     /// Unique identifier of the target user
    pub user_id: i32,
     /// A JSON-serialized object for new user permissions
    pub permissions: ChatPermissions,
     /// Date when restrictions will be lifted for the user, unix time. If user is restricted for more than 366 days or less than 30 seconds from the current time, they are considered to be restricted forever
    pub until_date: Option<i32>
}

impl RestrictChatMember {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "RestrictChatMember")
}}