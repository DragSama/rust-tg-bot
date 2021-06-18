use crate::types::{Chat, ChatInviteLink};
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "EditChatInviteLink does nothing until you `send` it"]
#[derive(Serialize)]
pub struct EditChatInviteLink<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// The invite link to edit
    pub invite_link: String,
    /// Point in time (Unix timestamp) when the link will expire
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i32>,
    /// Maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<i32>,
}

impl<'a> EditChatInviteLink<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64, invite_link: String) -> Self {
        Self {
            chat_id: chat_id,
            invite_link: invite_link,
            expire_date: None,
            member_limit: None,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<ChatInviteLink> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("editChatInviteLink", Some(string)).await?;
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
    pub fn expire_date(mut self, expire_date: i32) -> Self {
        self.expire_date = Some(expire_date);
        self
    }
    pub fn member_limit(mut self, member_limit: i32) -> Self {
        self.member_limit = Some(member_limit);
        self
    }
}
