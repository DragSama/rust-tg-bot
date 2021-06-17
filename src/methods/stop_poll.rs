use crate::types::{InlineKeyboardMarkup, Poll};
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "StopPoll does nothing until you `send` it"]
#[derive(Serialize)]
pub struct StopPoll<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i32,
    /// Identifier of the original message with the poll
    pub message_id: i32,
    /// A JSON-serialized object for a new message inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl<'a> StopPoll<'a> {
    pub fn new(bot: &'a Bot, chat_id: i32, message_id: i32) -> Self {
        Self {
            chat_id: chat_id,
            message_id: message_id,
            reply_markup: None,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<Poll> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("stopPoll", Some(string)).await?;
        Ok(serde_json::from_str::<Poll>(&resp.text().await?)?)
    }
    pub fn chat_id(mut self, chat_id: i32) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn message_id(mut self, message_id: i32) -> Self {
        self.message_id = message_id;
        self
    }
    pub fn reply_markup(mut self, reply_markup: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }
}
