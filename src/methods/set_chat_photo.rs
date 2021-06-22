use crate::types::{Chat, ChatPhoto, File, InputFile};
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "SetChatPhoto does nothing until you `send` it"]
#[derive(Serialize)]
pub struct SetChatPhoto<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// New chat photo, uploaded using multipart/form-data
    #[serde(skip)]
    pub photo: InputFile,
}

impl<'a> SetChatPhoto<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64, photo: InputFile) -> Self {
        Self {
            chat_id: chat_id,
            photo: photo,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<bool> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("setChatPhoto", Some(string)).await?;
        Ok(serde_json::from_str::<bool>(&resp.text().await?)?)
    }
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn photo(mut self, photo: InputFile) -> Self {
        self.photo = photo;
        self
    }
}
