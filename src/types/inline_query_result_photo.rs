use crate::types::{
    InlineKeyboardMarkup, InlineQuery, InlineQueryResult, InputMessageContent, Message,
    MessageEntity,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InlineQueryResultPhoto {
    /// Type of the result, must be photo
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL of the photo. Photo must be in jpeg format. Photo size must not exceed 5MB
    pub photo_url: String,
    /// URL of the thumbnail for the photo
    pub thumb_url: String,
    /// Optional. Width of the photo
    pub photo_width: Option<i32>,
    /// Optional. Height of the photo
    pub photo_height: Option<i32>,
    /// Optional. Title for the result
    pub title: Option<String>,
    /// Optional. Short description of the result
    pub description: Option<String>,
    /// Optional. Caption of the photo to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the photo
    pub input_message_content: Option<InputMessageContent>,
}
