use crate::types::{
    File, InlineKeyboardMarkup, InlineQuery, InlineQueryResult, InputMessageContent, Message,
    MessageEntity, Video,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct InlineQueryResultMpeg4Gif {
    /// Type of the result, must be mpeg4_gif
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the MP4 file. File size must not exceed 1MB
    pub mpeg4_url: String,
    /// Optional. Video width
    pub mpeg4_width: Option<i32>,
    /// Optional. Video height
    pub mpeg4_height: Option<i32>,
    /// Optional. Video duration
    pub mpeg4_duration: Option<i32>,
    /// URL of the static (JPEG or GIF) or animated (MPEG4) thumbnail for the result
    pub thumb_url: String,
    /// Optional. MIME type of the thumbnail, must be one of "image/jpeg", "image/gif", or "video/mp4". Defaults to "image/jpeg"
    pub thumb_mime_type: Option<String>,
    /// Optional. Title for the result
    pub title: Option<String>,
    /// Optional. Caption of the MPEG-4 file to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the caption. See formatting options for more details.
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the video animation
    pub input_message_content: Option<InputMessageContent>,
}
