use crate::types::{inline_keyboard_markup::InlineKeyboardMarkup, input_message_content::InputMessageContent, message::Message, message_entity::MessageEntity}

#[derive(Debug, Deserialize)]
pub struct InlineQueryResultDocument{
    pub type: String,
    pub id: String,
    pub title: String,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub document_url: String,
    pub mime_type: String,
    pub description: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
    pub thumb_url: Option<String>,
    pub thumb_width: Option<i32>,
    pub thumb_height: Option<i32>
}