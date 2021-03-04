use crate::types::{Chat, ChatMember};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct kickChatMember{
    pub chat_id: i32,
    pub user_id: i32,
    pub until_date: Option<i32>
}