use crate::types::Chat;
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "SetChatTitle does nothing until you `send` it"]
#[derive(Serialize)]
pub struct SetChatTitle<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// New chat title, 1-255 characters
    pub title: String,
}

impl<'a> SetChatTitle<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64, title: String) -> Self {
        Self {
            chat_id: chat_id,
            title: title,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<bool> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("setChatTitle", Some(string)).await?;
        Ok(serde_json::from_str::<bool>(&resp.text().await?)?)
    }
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }
}
