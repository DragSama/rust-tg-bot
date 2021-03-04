use crate::types::{Contact, InlineKeyboardMarkup, InlineQuery, InlineQueryResult, InputMessageContent, Message};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InlineQueryResultContact{
    pub r#type: String,
    pub id: String,
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub vcard: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
    pub thumb_url: Option<String>,
    pub thumb_width: Option<i32>,
    pub thumb_height: Option<i32>
}