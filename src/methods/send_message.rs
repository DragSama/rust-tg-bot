use crate::types::{InlineKeyboardMarkup, Message, MessageEntity};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct sendMessage{
    pub chat_id: i32,
    pub text: String,
    pub parse_mode: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub disable_web_page_preview: Option<bool>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i32>,
    pub allow_sending_without_reply: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>
}