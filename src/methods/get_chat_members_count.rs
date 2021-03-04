use crate::types::{Chat, ChatMember};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct getChatMembersCount{
    pub chat_id: i32
}