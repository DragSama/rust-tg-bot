use crate::types::{Chat};
use serde::Serialize;
use serde_json;


#[derive(Debug, Serialize)]
pub struct ExportChatInviteLink{
     /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i32
}

impl ExportChatInviteLink {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "ExportChatInviteLink")
}}