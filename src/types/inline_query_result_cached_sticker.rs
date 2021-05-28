use crate::types::{
    InlineKeyboardMarkup, InlineQuery, InlineQueryResult, InputMessageContent, Message, Sticker,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct InlineQueryResultCachedSticker {
    /// Type of the result, must be sticker
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier of the sticker
    pub sticker_file_id: String,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the sticker
    pub input_message_content: Option<InputMessageContent>,
}
