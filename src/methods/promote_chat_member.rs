use crate::types::{Chat, ChatMember};
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "PromoteChatMember does nothing until you `send` it"]
#[derive(Serialize)]
pub struct PromoteChatMember<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// Unique identifier of the target user
    pub user_id: i32,
    /// Pass True, if the administrator's presence in the chat is hidden
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    /// Pass True, if the administrator can access the chat event log, chat statistics, message statistics in channels, see channel members, see anonymous administrators in supergroups and ignore slow mode. Implied by any other administrator privilege
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_chat: Option<bool>,
    /// Pass True, if the administrator can create channel posts, channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,
    /// Pass True, if the administrator can edit messages of other users and can pin messages, channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,
    /// Pass True, if the administrator can delete messages of other users
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_messages: Option<bool>,
    /// Pass True, if the administrator can manage voice chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_voice_chats: Option<bool>,
    /// Pass True, if the administrator can restrict, ban or unban chat members
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_restrict_members: Option<bool>,
    /// Pass True, if the administrator can add new administrators with a subset of their own privileges or demote administrators that he has promoted, directly or indirectly (promoted by administrators that were appointed by him)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_promote_members: Option<bool>,
    /// Pass True, if the administrator can change chat title, photo and other settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,
    /// Pass True, if the administrator can invite new users to the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,
    /// Pass True, if the administrator can pin messages, supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
}

impl<'a> PromoteChatMember<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64, user_id: i32) -> Self {
        Self {
            chat_id: chat_id,
            user_id: user_id,
            is_anonymous: None,
            can_manage_chat: None,
            can_post_messages: None,
            can_edit_messages: None,
            can_delete_messages: None,
            can_manage_voice_chats: None,
            can_restrict_members: None,
            can_promote_members: None,
            can_change_info: None,
            can_invite_users: None,
            can_pin_messages: None,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<bool> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("promoteChatMember", Some(string)).await?;
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
    pub fn is_anonymous(mut self, is_anonymous: bool) -> Self {
        self.is_anonymous = Some(is_anonymous);
        self
    }
    pub fn can_manage_chat(mut self, can_manage_chat: bool) -> Self {
        self.can_manage_chat = Some(can_manage_chat);
        self
    }
    pub fn can_post_messages(mut self, can_post_messages: bool) -> Self {
        self.can_post_messages = Some(can_post_messages);
        self
    }
    pub fn can_edit_messages(mut self, can_edit_messages: bool) -> Self {
        self.can_edit_messages = Some(can_edit_messages);
        self
    }
    pub fn can_delete_messages(mut self, can_delete_messages: bool) -> Self {
        self.can_delete_messages = Some(can_delete_messages);
        self
    }
    pub fn can_manage_voice_chats(mut self, can_manage_voice_chats: bool) -> Self {
        self.can_manage_voice_chats = Some(can_manage_voice_chats);
        self
    }
    pub fn can_restrict_members(mut self, can_restrict_members: bool) -> Self {
        self.can_restrict_members = Some(can_restrict_members);
        self
    }
    pub fn can_promote_members(mut self, can_promote_members: bool) -> Self {
        self.can_promote_members = Some(can_promote_members);
        self
    }
    pub fn can_change_info(mut self, can_change_info: bool) -> Self {
        self.can_change_info = Some(can_change_info);
        self
    }
    pub fn can_invite_users(mut self, can_invite_users: bool) -> Self {
        self.can_invite_users = Some(can_invite_users);
        self
    }
    pub fn can_pin_messages(mut self, can_pin_messages: bool) -> Self {
        self.can_pin_messages = Some(can_pin_messages);
        self
    }
}
