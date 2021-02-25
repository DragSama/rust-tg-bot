use crate::types::{inline_keyboard_markup::InlineKeyboardMarkup, message::Message}

#[derive(Debug, Serialize)]
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
    pub thumb_width: Option<i64>,
    pub thumb_height: Option<i64>
}