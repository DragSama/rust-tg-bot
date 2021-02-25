use crate::types::{inline_keyboard_markup::InlineKeyboardMarkup, message::Message, message_entity::MessageEntity}

#[derive(Debug, Serialize)]
pub struct InlineQueryResultVideo{
    pub type: String,
    pub id: String,
    pub video_url: String,
    pub mime_type: String,
    pub thumb_url: String,
    pub title: String,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub video_width: Option<i64>,
    pub video_height: Option<i64>,
    pub video_duration: Option<i64>,
    pub description: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>
}