use crate::types::{message::Message, message_entity::MessageEntity}

#[derive(Debug, Serialize)]
pub struct InputMediaPhoto{
    pub type: String,
    pub media: String,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>
}