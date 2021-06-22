use crate::types::{
    Audio, File, InlineKeyboardMarkup, InputFile, Message, MessageEntity, User, Voice,
};
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "SendVoice does nothing until you `send` it"]
#[derive(Serialize)]
pub struct SendVoice<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// Audio file to send. Pass a file_id as String to send a file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More info on Sending Files
    #[serde(skip)]
    pub voice: InputFile,
    /// Voice message caption, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the voice message caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Duration of the voice message in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i32>,
    /// Pass True, if the message should be sent even if the specified replied-to message is not found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl<'a> SendVoice<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64, voice: InputFile) -> Self {
        Self {
            chat_id: chat_id,
            voice: voice,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            duration: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<Message> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("sendVoice", Some(string)).await?;
        Ok(serde_json::from_str::<Message>(&resp.text().await?)?)
    }
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn voice(mut self, voice: InputFile) -> Self {
        self.voice = voice;
        self
    }
    pub fn caption(mut self, caption: String) -> Self {
        self.caption = Some(caption);
        self
    }
    pub fn parse_mode(mut self, parse_mode: String) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }
    pub fn caption_entities(mut self, caption_entities: Vec<MessageEntity>) -> Self {
        self.caption_entities = Some(caption_entities);
        self
    }
    pub fn duration(mut self, duration: i32) -> Self {
        self.duration = Some(duration);
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
    pub fn reply_markup(mut self, reply_markup: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }
}
