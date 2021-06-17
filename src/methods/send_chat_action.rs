use crate::types::Chat;
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "SendChatAction does nothing until you `send` it"]
#[derive(Serialize)]
pub struct SendChatAction<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i32,
    /// Type of action to broadcast. Choose one, depending on what the user is about to receive: typing for text messages, upload_photo for photos, record_video or upload_video for videos, record_voice or upload_voice for voice notes, upload_document for general files, find_location for location data, record_video_note or upload_video_note for video notes.
    pub action: String,
}

impl<'a> SendChatAction<'a> {
    pub fn new(bot: &'a Bot, chat_id: i32, action: String) -> Self {
        Self {
            chat_id: chat_id,
            action: action,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<bool> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("sendChatAction", Some(string)).await?;
        Ok(serde_json::from_str::<bool>(&resp.text().await?)?)
    }
    pub fn chat_id(mut self, chat_id: i32) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn action(mut self, action: String) -> Self {
        self.action = action;
        self
    }
}
