use crate::types::{InlineKeyboardMarkup, InlineQuery, InlineQueryResult, InputMessageContent, Message, MessageEntity, Video};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct InlineQueryResultCachedVideo{
    /// Type of the result, must be video
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the video file
    pub video_file_id: String,
    /// Title for the result
    pub title: String,
    /// Optional. Short description of the result
    pub description: Option<String>,
    /// Optional. Caption of the video to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the video caption. See formatting options for more details.
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the video
    pub input_message_content: Option<InputMessageContent>
}