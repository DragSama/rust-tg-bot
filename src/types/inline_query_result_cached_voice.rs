use crate::types::{inline_keyboard_markup::InlineKeyboardMarkup, message::Message, message_entity::MessageEntity}

#[derive(Debug, Serialize)]
pub struct InlineQueryResultCachedVoice{
    pub type: String,
    pub id: String,
    pub voice_file_id: String,
    pub title: String,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>
}