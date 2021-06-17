use crate::types::{Chat, Message};
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "UnpinChatMessage does nothing until you `send` it"]
#[derive(Serialize)]
pub struct UnpinChatMessage<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i32,
    /// Identifier of a message to unpin. If not specified, the most recent pinned message (by sending date) will be unpinned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,
}

impl<'a> UnpinChatMessage<'a> {
    pub fn new(bot: &'a Bot, chat_id: i32) -> Self {
        Self {
            chat_id: chat_id,
            message_id: None,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<bool> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("unpinChatMessage", Some(string)).await?;
        Ok(serde_json::from_str::<bool>(&resp.text().await?)?)
    }
    pub fn chat_id(mut self, chat_id: i32) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn message_id(mut self, message_id: i32) -> Self {
        self.message_id = Some(message_id);
        self
    }
}
