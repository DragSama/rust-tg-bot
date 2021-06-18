use crate::types::{Chat, ChatMember, ChatPermissions};
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "RestrictChatMember does nothing until you `send` it"]
#[derive(Serialize)]
pub struct RestrictChatMember<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: i64,
    /// Unique identifier of the target user
    pub user_id: i32,
    /// A JSON-serialized object for new user permissions
    pub permissions: ChatPermissions,
    /// Date when restrictions will be lifted for the user, unix time. If user is restricted for more than 366 days or less than 30 seconds from the current time, they are considered to be restricted forever
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i32>,
}

impl<'a> RestrictChatMember<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64, user_id: i32, permissions: ChatPermissions) -> Self {
        Self {
            chat_id: chat_id,
            user_id: user_id,
            permissions: permissions,
            until_date: None,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<bool> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("restrictChatMember", Some(string)).await?;
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
    pub fn permissions(mut self, permissions: ChatPermissions) -> Self {
        self.permissions = permissions;
        self
    }
    pub fn until_date(mut self, until_date: i32) -> Self {
        self.until_date = Some(until_date);
        self
    }
}
