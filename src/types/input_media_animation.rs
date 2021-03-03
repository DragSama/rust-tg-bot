use crate::types::{file::File, input_file::InputFile, message::Message, message_entity::MessageEntity}

#[derive(Debug, Deserialize)]
pub struct InputMediaAnimation{
    pub type: String,
    pub media: String,
    pub thumb: Option<InputFile>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration: Option<i32>
}