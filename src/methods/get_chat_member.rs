use crate::types::{Chat, ChatMember};
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "GetChatMember does nothing until you `send` it"]
#[derive(Serialize)]
pub struct GetChatMember<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: i32,
    /// Unique identifier of the target user
    pub user_id: i32,
}

impl<'a> GetChatMember<'a> {
    pub fn new(bot: &'a Bot, chat_id: i32, user_id: i32) -> Self {
        Self {
            chat_id: chat_id,
            user_id: user_id,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<ChatMember> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("getChatMember", Some(string)).await?;
        Ok(serde_json::from_str::<ChatMember>(&resp.text().await?)?)
    }
    pub fn chat_id(mut self, chat_id: i32) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn user_id(mut self, user_id: i32) -> Self {
        self.user_id = user_id;
        self
    }
}
