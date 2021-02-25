use crate::types::{inline_keyboard_markup::InlineKeyboardMarkup, message::Message, message_entity::MessageEntity}

#[derive(Debug, Deserialize)]
pub struct InlineQueryResultAudio{
    pub type: String,
    pub id: String,
    pub audio_url: String,
    pub title: String,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub performer: Option<String>,
    pub audio_duration: Option<i64>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>
}