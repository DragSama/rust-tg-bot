use crate::types::{InlineKeyboardMarkup, Poll};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct stopPoll{
    pub chat_id: i32,
    pub message_id: i32,
    pub reply_markup: Option<InlineKeyboardMarkup>
}