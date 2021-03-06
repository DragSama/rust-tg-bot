use crate::types::{
    InlineKeyboardMarkup, InlineQuery, InlineQueryResult, InputMessageContent, Message,
    MessageEntity, Video,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InlineQueryResultVideo {
    /// Type of the result, must be video
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the embedded video player or video file
    pub video_url: String,
    /// Mime type of the content of video url, "text/html" or "video/mp4"
    pub mime_type: String,
    /// URL of the thumbnail (jpeg only) for the video
    pub thumb_url: String,
    /// Title for the result
    pub title: String,
    /// Optional. Caption of the video to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the video caption. See formatting options for more details.
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Video width
    pub video_width: Option<i32>,
    /// Optional. Video height
    pub video_height: Option<i32>,
    /// Optional. Video duration in seconds
    pub video_duration: Option<i32>,
    /// Optional. Short description of the result
    pub description: Option<String>,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the video. This field is required if InlineQueryResultVideo is used to send an HTML-page as a result (e.g., a YouTube video).
    pub input_message_content: Option<InputMessageContent>,
}
