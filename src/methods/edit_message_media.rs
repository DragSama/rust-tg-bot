use crate::types::{InlineKeyboardMarkup, InputMedia, Message};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct editMessageMedia{
    pub chat_id: Option<i32>,
    pub message_id: Option<i32>,
    pub inline_message_id: Option<String>,
    pub media: InputMedia,
    pub reply_markup: Option<InlineKeyboardMarkup>
}