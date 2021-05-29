use crate::types::{Message, MessageEntity};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InputTextMessageContent {
    /// Text of the message to be sent, 1-4096 characters
    pub message_text: String,
    /// Optional. Mode for parsing entities in the message text. See formatting options for more details.
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in message text, which can be specified instead of parse_mode
    pub entities: Option<Vec<MessageEntity>>,
    /// Optional. Disables link previews for links in the sent message
    pub disable_web_page_preview: Option<bool>,
}
