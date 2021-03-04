use crate::types::{Chat, Message};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct unpinAllChatMessages{
    pub chat_id: i32
}