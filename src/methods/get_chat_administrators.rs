use crate::types::{Chat, ChatMember};
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "GetChatAdministrators does nothing until you `send` it"]
#[derive(Serialize)]
pub struct GetChatAdministrators<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: i32,
}

impl<'a> GetChatAdministrators<'a> {
    pub fn new(bot: &'a Bot, chat_id: i32) -> Self {
        Self {
            chat_id: chat_id,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<Vec<ChatMember>> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("getChatAdministrators", Some(string)).await?;
        Ok(serde_json::from_str::<Vec<ChatMember>>(
            &resp.text().await?,
        )?)
    }
    pub fn chat_id(mut self, chat_id: i32) -> Self {
        self.chat_id = chat_id;
        self
    }
}
