use crate::types::{inline_keyboard_markup::InlineKeyboardMarkup, message::Message}

#[derive(Debug, Serialize)]
pub struct InlineQueryResultContact{
    pub type: String,
    pub id: String,
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub vcard: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
    pub thumb_url: Option<String>,
    pub thumb_width: Option<i64>,
    pub thumb_height: Option<i64>
}