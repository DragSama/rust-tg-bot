use crate::types::{Message};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct deleteMessage{
    pub chat_id: i32,
    pub message_id: i32
}