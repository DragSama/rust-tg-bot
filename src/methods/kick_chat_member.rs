use crate::types::{Chat, ChatMember};
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct KickChatMember {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: i32,
    /// Unique identifier of the target user
    pub user_id: i32,
    /// Date when the user will be unbanned, unix time. If user is banned for more than 366 days or less than 30 seconds from the current time they are considered to be banned forever. Applied for supergroups and channels only.
    pub until_date: Option<i32>,
}

impl KickChatMember {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "KickChatMember")
    }
}
