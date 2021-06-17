use crate::types::{Message, User};
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "ForwardMessage does nothing until you `send` it"]
#[derive(Serialize)]
pub struct ForwardMessage<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i32,
    /// Unique identifier for the chat where the original message was sent (or channel username in the format @channelusername)
    pub from_chat_id: i32,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Message identifier in the chat specified in from_chat_id
    pub message_id: i32,
}

impl<'a> ForwardMessage<'a> {
    pub fn new(bot: &'a Bot, chat_id: i32, from_chat_id: i32, message_id: i32) -> Self {
        Self {
            chat_id: chat_id,
            from_chat_id: from_chat_id,
            message_id: message_id,
            disable_notification: None,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<Message> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("forwardMessage", Some(string)).await?;
        Ok(serde_json::from_str::<Message>(&resp.text().await?)?)
    }
    pub fn chat_id(mut self, chat_id: i32) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn from_chat_id(mut self, from_chat_id: i32) -> Self {
        self.from_chat_id = from_chat_id;
        self
    }
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }
    pub fn message_id(mut self, message_id: i32) -> Self {
        self.message_id = message_id;
        self
    }
}
