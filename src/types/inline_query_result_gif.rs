use crate::types::{
    File, InlineKeyboardMarkup, InlineQuery, InlineQueryResult, InputMessageContent, Message,
    MessageEntity,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct InlineQueryResultGif {
    /// Type of the result, must be gif
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the GIF file. File size must not exceed 1MB
    pub gif_url: String,
    /// Optional. Width of the GIF
    pub gif_width: Option<i32>,
    /// Optional. Height of the GIF
    pub gif_height: Option<i32>,
    /// Optional. Duration of the GIF
    pub gif_duration: Option<i32>,
    /// URL of the static (JPEG or GIF) or animated (MPEG4) thumbnail for the result
    pub thumb_url: String,
    /// Optional. MIME type of the thumbnail, must be one of "image/jpeg", "image/gif", or "video/mp4". Defaults to "image/jpeg"
    pub thumb_mime_type: Option<String>,
    /// Optional. Title for the result
    pub title: Option<String>,
    /// Optional. Caption of the GIF file to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the caption. See formatting options for more details.
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the GIF animation
    pub input_message_content: Option<InputMessageContent>,
}
