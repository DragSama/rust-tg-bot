use crate::types::{Chat, Message};
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "UnpinAllChatMessages does nothing until you `send` it"]
#[derive(Serialize)]
pub struct UnpinAllChatMessages<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
}

impl<'a> UnpinAllChatMessages<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64) -> Self {
        Self {
            chat_id: chat_id,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<bool> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("unpinAllChatMessages", Some(string)).await?;
        Ok(serde_json::from_str::<bool>(&resp.text().await?)?)
    }
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
}
