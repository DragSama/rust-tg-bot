use crate::types::{inline_keyboard_markup::InlineKeyboardMarkup, input_message_content::InputMessageContent, message::Message, message_entity::MessageEntity}

#[derive(Debug, Deserialize)]
pub struct InlineQueryResultMpeg4Gif{
    pub type: String,
    pub id: String,
    pub mpeg4_url: String,
    pub mpeg4_width: Option<i32>,
    pub mpeg4_height: Option<i32>,
    pub mpeg4_duration: Option<i32>,
    pub thumb_url: String,
    pub thumb_mime_type: Option<String>,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>
}