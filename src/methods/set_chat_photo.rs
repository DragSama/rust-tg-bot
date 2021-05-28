use crate::types::{Chat, ChatPhoto, File, InputFile};
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct SetChatPhoto {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i32,
    /// New chat photo, uploaded using multipart/form-data
    pub photo: InputFile,
}

impl SetChatPhoto {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "SetChatPhoto")
    }
}
