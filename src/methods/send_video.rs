use crate::types::{File, InlineKeyboardMarkup, InputFile, Message, MessageEntity, Video};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct sendVideo{
    pub chat_id: i32,
    pub video: InputFile,
    pub duration: Option<i32>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub thumb: Option<InputFile>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub supports_streaming: Option<bool>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i32>,
    pub allow_sending_without_reply: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>
}