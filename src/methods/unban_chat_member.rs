use crate::types::{Chat, ChatMember};
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "UnbanChatMember does nothing until you `send` it"]
#[derive(Serialize)]
pub struct UnbanChatMember<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format @username)
    pub chat_id: i32,
    /// Unique identifier of the target user
    pub user_id: i32,
    /// Do nothing if the user is not banned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_if_banned: Option<bool>,
}

impl<'a> UnbanChatMember<'a> {
    pub fn new(bot: &'a Bot, chat_id: i32, user_id: i32) -> Self {
        Self {
            chat_id: chat_id,
            user_id: user_id,
            only_if_banned: None,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<bool> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("unbanChatMember", Some(string)).await?;
        Ok(serde_json::from_str::<bool>(&resp.text().await?)?)
    }
    pub fn chat_id(mut self, chat_id: i32) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn user_id(mut self, user_id: i32) -> Self {
        self.user_id = user_id;
        self
    }
    pub fn only_if_banned(mut self, only_if_banned: bool) -> Self {
        self.only_if_banned = Some(only_if_banned);
        self
    }
}
