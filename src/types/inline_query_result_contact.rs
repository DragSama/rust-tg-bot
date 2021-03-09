use crate::types::{Contact, InlineKeyboardMarkup, InlineQuery, InlineQueryResult, InputMessageContent, Message};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct InlineQueryResultContact{
    /// Type of the result, must be contact
    pub r#type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Optional. Contact's last name
    pub last_name: Option<String>,
    /// Optional. Additional data about the contact in the form of a vCard, 0-2048 bytes
    pub vcard: Option<String>,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the contact
    pub input_message_content: Option<InputMessageContent>,
    /// Optional. Url of the thumbnail for the result
    pub thumb_url: Option<String>,
    /// Optional. Thumbnail width
    pub thumb_width: Option<i32>,
    /// Optional. Thumbnail height
    pub thumb_height: Option<i32>
}