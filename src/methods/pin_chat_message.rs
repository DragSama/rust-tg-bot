use crate::types::{Chat, Message};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct pinChatMessage{
    pub chat_id: i32,
    pub message_id: i32,
    pub disable_notification: Option<bool>
}