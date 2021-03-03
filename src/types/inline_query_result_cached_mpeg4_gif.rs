use crate::types::{InlineKeyboardMarkup, InputMessageContent, Message, MessageEntity};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InlineQueryResultCachedMpeg4Gif{
    pub type: String,
    pub id: String,
    pub mpeg4_file_id: String,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>
}