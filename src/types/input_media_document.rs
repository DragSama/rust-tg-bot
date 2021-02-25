use crate::types::{file::File, message::Message, message_entity::MessageEntity}

#[derive(Debug, Deserialize)]
pub struct InputMediaDocument{
    pub type: String,
    pub media: String,
    pub thumb: Option<InputFile>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub disable_content_type_detection: Option<bool>
}