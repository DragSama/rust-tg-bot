use crate::types::{Message};
use serde::Serialize;
use serde_json;


#[derive(Debug, Serialize)]
pub struct DeleteMessage{
     /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i32,
     /// Identifier of the message to delete
    pub message_id: i32
}

impl DeleteMessage {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "DeleteMessage")
}}