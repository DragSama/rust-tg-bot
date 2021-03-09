use crate::types::{InlineKeyboardMarkup, InlineQuery, InlineQueryResult, InputMessageContent, Message, MessageEntity, Voice};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct InlineQueryResultCachedVoice{
    /// Type of the result, must be voice
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the voice message
    pub voice_file_id: String,
    /// Voice message title
    pub title: String,
    /// Optional. Caption, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the voice message caption. See formatting options for more details.
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the voice message
    pub input_message_content: Option<InputMessageContent>
}