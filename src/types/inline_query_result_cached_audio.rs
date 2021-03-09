use crate::types::{Audio, InlineKeyboardMarkup, InlineQuery, InlineQueryResult, InputMessageContent, Message, MessageEntity};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct InlineQueryResultCachedAudio{
    /// Type of the result, must be audio
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the audio file
    pub audio_file_id: String,
    /// Optional. Caption, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the audio caption. See formatting options for more details.
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the audio
    pub input_message_content: Option<InputMessageContent>
}