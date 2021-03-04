use crate::types::{Chat, Message};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct unpinChatMessage{
    pub chat_id: i32,
    pub message_id: Option<i32>
}