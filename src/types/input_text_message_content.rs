use crate::types::{message_entity::MessageEntity}

#[derive(Debug, Serialize)]
pub struct InputTextMessageContent{
    pub message_text: String,
    pub parse_mode: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub disable_web_page_preview: Option<bool>
}