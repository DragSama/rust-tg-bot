use crate::types::{inline_keyboard_markup::InlineKeyboardMarkup, input_message_content::InputMessageContent, message::Message, message_entity::MessageEntity}

#[derive(Debug, Deserialize)]
pub struct InlineQueryResultVoice{
    pub type: String,
    pub id: String,
    pub voice_url: String,
    pub title: String,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub voice_duration: Option<i32>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>
}