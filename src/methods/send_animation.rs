use crate::types::{
    Animation, File, InlineKeyboardMarkup, InputFile, Message, MessageEntity, User,
};
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "SendAnimation does nothing until you `send` it"]
#[derive(Serialize)]
pub struct SendAnimation<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// Animation to send. Pass a file_id as String to send an animation that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get an animation from the Internet, or upload a new animation using multipart/form-data. More info on Sending Files
    #[serde(skip)]
    pub animation: InputFile,
    /// Duration of sent animation in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    /// Animation width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    /// Animation height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass "attach://<file_attach_name>" if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More info on Sending Files
    #[serde(skip)]
    pub thumb: Option<InputFile>,
    /// Animation caption (may also be used when resending animation by file_id), 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the animation caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
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

impl<'a> SendAnimation<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64, animation: InputFile) -> Self {
        Self {
            chat_id: chat_id,
            animation: animation,
            duration: None,
            width: None,
            height: None,
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<Message> {
        let builder = self.bot.get_builder("sendAnimation").query(&self);
        let mut query = Vec::new();
        let mut form = reqwest::multipart::Form::new();
        match self.animation {
            InputFile::File(content) => {
                let file = reqwest::multipart::Part::bytes(content).file_name("animation");
                form = form.part("animation", file);
            },
            InputFile::Url(url) => {
                query.push(("animation", url))
            }
        }
        let resp = builder.query(&query)
            .multipart(form)
            .send()
            .await?
            .text()
            .await?;
        println!("{}", resp);
        Ok(serde_json::from_str::<Message>(&resp)?)
    }
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn animation(mut self, animation: InputFile) -> Self {
        self.animation = animation;
        self
    }
    pub fn duration(mut self, duration: i32) -> Self {
        self.duration = Some(duration);
        self
    }
    pub fn width(mut self, width: i32) -> Self {
        self.width = Some(width);
        self
    }
    pub fn height(mut self, height: i32) -> Self {
        self.height = Some(height);
        self
    }
    pub fn thumb(mut self, thumb: InputFile) -> Self {
        self.thumb = Some(thumb);
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
