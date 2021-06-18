use crate::types::{Chat, ChatInviteLink};
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "RevokeChatInviteLink does nothing until you `send` it"]
#[derive(Serialize)]
pub struct RevokeChatInviteLink<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier of the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// The invite link to revoke
    pub invite_link: String,
}

impl<'a> RevokeChatInviteLink<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64, invite_link: String) -> Self {
        Self {
            chat_id: chat_id,
            invite_link: invite_link,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<ChatInviteLink> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("revokeChatInviteLink", Some(string)).await?;
        Ok(serde_json::from_str::<ChatInviteLink>(&resp.text().await?)?)
    }
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn invite_link(mut self, invite_link: String) -> Self {
        self.invite_link = invite_link;
        self
    }
}
