use crate::types::{Chat, ChatPhoto};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct deleteChatPhoto{
    pub chat_id: i32
}