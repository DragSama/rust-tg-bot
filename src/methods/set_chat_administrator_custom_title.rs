use crate::types::Chat;
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "SetChatAdministratorCustomTitle does nothing until you `send` it"]
#[derive(Serialize)]
pub struct SetChatAdministratorCustomTitle<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: i64,
    /// Unique identifier of the target user
    pub user_id: i32,
    /// New custom title for the administrator; 0-16 characters, emoji are not allowed
    pub custom_title: String,
}

impl<'a> SetChatAdministratorCustomTitle<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64, user_id: i32, custom_title: String) -> Self {
        Self {
            chat_id: chat_id,
            user_id: user_id,
            custom_title: custom_title,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<bool> {
        let string = serde_json::to_string(&self)?;
        let resp = self
            .bot
            .send("setChatAdministratorCustomTitle", Some(string))
            .await?;
        Ok(serde_json::from_str::<bool>(&resp.text().await?)?)
    }
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn user_id(mut self, user_id: i32) -> Self {
        self.user_id = user_id;
        self
    }
    pub fn custom_title(mut self, custom_title: String) -> Self {
        self.custom_title = custom_title;
        self
    }
}
