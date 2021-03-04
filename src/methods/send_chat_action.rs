use crate::types::{Chat};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct sendChatAction{
    pub chat_id: i32,
    pub action: String
}