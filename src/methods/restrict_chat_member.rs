use crate::types::{Chat, ChatMember, ChatPermissions};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct restrictChatMember{
    pub chat_id: i32,
    pub user_id: i32,
    pub permissions: ChatPermissions,
    pub until_date: Option<i32>
}