use crate::types::{Chat};
use serde::Serialize;
use serde_json;


#[derive(Debug, Serialize)]
pub struct SendChatAction{
     /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i32,
     /// Type of action to broadcast. Choose one, depending on what the user is about to receive: typing for text messages, upload_photo for photos, record_video or upload_video for videos, record_voice or upload_voice for voice notes, upload_document for general files, find_location for location data, record_video_note or upload_video_note for video notes.
    pub action: String
}

impl SendChatAction {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "SendChatAction")
}}