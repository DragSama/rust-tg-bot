use crate::types::{Chat, ChatMember};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct getChatMember{
    pub chat_id: i32,
    pub user_id: i32
}