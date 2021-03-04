use crate::types::{File, InlineKeyboardMarkup, InputFile, Message, MessageEntity, Voice};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct sendVoice{
    pub chat_id: i32,
    pub voice: InputFile,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub duration: Option<i32>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i32>,
    pub allow_sending_without_reply: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>
}