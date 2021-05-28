use crate::types::Chat;
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct SetChatAdministratorCustomTitle {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: i32,
    /// Unique identifier of the target user
    pub user_id: i32,
    /// New custom title for the administrator; 0-16 characters, emoji are not allowed
    pub custom_title: String,
}

impl SetChatAdministratorCustomTitle {
    pub fn get_data(&self) -> (String, &str) {
        (
            serde_json::to_string(&self).unwrap(),
            "SetChatAdministratorCustomTitle",
        )
    }
}
