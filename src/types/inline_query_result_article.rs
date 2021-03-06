use crate::types::{
    InlineKeyboardMarkup, InlineQuery, InlineQueryResult, InputMessageContent, Message,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InlineQueryResultArticle {
    /// Type of the result, must be article
    pub r#type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Title of the result
    pub title: String,
    /// Content of the message to be sent
    pub input_message_content: InputMessageContent,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. URL of the result
    pub url: Option<String>,
    /// Optional. Pass True, if you don't want the URL to be shown in the message
    pub hide_url: Option<bool>,
    /// Optional. Short description of the result
    pub description: Option<String>,
    /// Optional. Url of the thumbnail for the result
    pub thumb_url: Option<String>,
    /// Optional. Thumbnail width
    pub thumb_width: Option<i32>,
    /// Optional. Thumbnail height
    pub thumb_height: Option<i32>,
}
