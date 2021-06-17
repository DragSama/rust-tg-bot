use crate::types::{Chat, Message};
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "PinChatMessage does nothing until you `send` it"]
#[derive(Serialize)]
pub struct PinChatMessage<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i32,
    /// Identifier of a message to pin
    pub message_id: i32,
    /// Pass True, if it is not necessary to send a notification to all chat members about the new pinned message. Notifications are always disabled in channels and private chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
}

impl<'a> PinChatMessage<'a> {
    pub fn new(bot: &'a Bot, chat_id: i32, message_id: i32) -> Self {
        Self {
            chat_id: chat_id,
            message_id: message_id,
            disable_notification: None,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<bool> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("pinChatMessage", Some(string)).await?;
        Ok(serde_json::from_str::<bool>(&resp.text().await?)?)
    }
    pub fn chat_id(mut self, chat_id: i32) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn message_id(mut self, message_id: i32) -> Self {
        self.message_id = message_id;
        self
    }
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }
}
