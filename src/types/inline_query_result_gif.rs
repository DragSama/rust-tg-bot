use crate::types::{InlineKeyboardMarkup, InlineQuery, InlineQueryResult, InputMessageContent, Message, MessageEntity};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InlineQueryResultGif{
    pub r#type: String,
    pub id: String,
    pub gif_url: String,
    pub gif_width: Option<i32>,
    pub gif_height: Option<i32>,
    pub gif_duration: Option<i32>,
    pub thumb_url: String,
    pub thumb_mime_type: Option<String>,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>
}