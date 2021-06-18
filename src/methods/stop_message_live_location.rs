use crate::types::{InlineKeyboardMarkup, Location, Message};
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "StopMessageLiveLocation does nothing until you `send` it"]
#[derive(Serialize)]
pub struct StopMessageLiveLocation<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// Required if inline_message_id is not specified. Identifier of the message with live location to stop
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// A JSON-serialized object for a new inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl<'a> StopMessageLiveLocation<'a> {
    pub fn new(bot: &'a Bot) -> Self {
        Self {
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            reply_markup: None,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<Message> {
        let string = serde_json::to_string(&self)?;
        let resp = self
            .bot
            .send("stopMessageLiveLocation", Some(string))
            .await?;
        Ok(serde_json::from_str::<Message>(&resp.text().await?)?)
    }
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = Some(chat_id);
        self
    }
    pub fn message_id(mut self, message_id: i32) -> Self {
        self.message_id = Some(message_id);
        self
    }
    pub fn inline_message_id(mut self, inline_message_id: String) -> Self {
        self.inline_message_id = Some(inline_message_id);
        self
    }
    pub fn reply_markup(mut self, reply_markup: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }
}
