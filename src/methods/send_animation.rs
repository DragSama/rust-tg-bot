use crate::types::{Animation, File, InlineKeyboardMarkup, InputFile, Message, MessageEntity};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct sendAnimation{
    pub chat_id: i32,
    pub animation: InputFile,
    pub duration: Option<i32>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub thumb: Option<InputFile>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i32>,
    pub allow_sending_without_reply: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>
}