use crate::types::{Document, InlineKeyboardMarkup, InlineQuery, InlineQueryResult, InputMessageContent, Message, MessageEntity};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct InlineQueryResultDocument{
    /// Type of the result, must be document
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// Title for the result
    pub title: String,
    /// Optional. Caption of the document to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the document caption. See formatting options for more details.
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// A valid URL for the file
    pub document_url: String,
    /// Mime type of the content of the file, either "application/pdf" or "application/zip"
    pub mime_type: String,
    /// Optional. Short description of the result
    pub description: Option<String>,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the file
    pub input_message_content: Option<InputMessageContent>,
    /// Optional. URL of the thumbnail (jpeg only) for the file
    pub thumb_url: Option<String>,
    /// Optional. Thumbnail width
    pub thumb_width: Option<i32>,
    /// Optional. Thumbnail height
    pub thumb_height: Option<i32>
}