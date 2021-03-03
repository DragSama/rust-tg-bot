use crate::types::{inline_keyboard_markup::InlineKeyboardMarkup, input_message_content::InputMessageContent, message::Message, message_entity::MessageEntity}

#[derive(Debug, Deserialize)]
pub struct InlineQueryResultPhoto{
    pub type: String,
    pub id: String,
    pub photo_url: String,
    pub thumb_url: String,
    pub photo_width: Option<i32>,
    pub photo_height: Option<i32>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>
}