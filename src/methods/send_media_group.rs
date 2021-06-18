use crate::types::{Audio, InputMedia, InputMediaAudio, Message, User};
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "SendMediaGroup does nothing until you `send` it"]
#[derive(Serialize)]
pub struct SendMediaGroup<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// A JSON-serialized array describing messages to be sent, must include 2-10 items
    pub media: Vec<InputMediaAudio>,
    /// Sends messages silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// If the messages are a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i32>,
    /// Pass True, if the message should be sent even if the specified replied-to message is not found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
}

impl<'a> SendMediaGroup<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64, media: Vec<InputMediaAudio>) -> Self {
        Self {
            chat_id: chat_id,
            media: media,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<Vec<Message>> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("sendMediaGroup", Some(string)).await?;
        Ok(serde_json::from_str::<Vec<Message>>(&resp.text().await?)?)
    }
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn media(mut self, media: Vec<InputMediaAudio>) -> Self {
        self.media = media;
        self
    }
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }
    pub fn reply_to_message_id(mut self, reply_to_message_id: i32) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }
    pub fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }
}
