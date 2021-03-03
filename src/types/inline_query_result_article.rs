use crate::types::{InlineKeyboardMarkup, InputMessageContent, Message};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InlineQueryResultArticle{
    pub type: String,
    pub id: String,
    pub title: String,
    pub input_message_content: InputMessageContent,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub url: Option<String>,
    pub hide_url: Option<bool>,
    pub description: Option<String>,
    pub thumb_url: Option<String>,
    pub thumb_width: Option<i32>,
    pub thumb_height: Option<i32>
}