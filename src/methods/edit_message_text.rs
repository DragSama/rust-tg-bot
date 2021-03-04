use crate::types::{InlineKeyboardMarkup, Message, MessageEntity};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct editMessageText{
    pub chat_id: Option<i32>,
    pub message_id: Option<i32>,
    pub inline_message_id: Option<String>,
    pub text: String,
    pub parse_mode: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub disable_web_page_preview: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>
}