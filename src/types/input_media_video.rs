use crate::types::{File, InputFile, InputMedia, Message, MessageEntity, Video};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InputMediaVideo{
    pub r#type: String,
    pub media: String,
    pub thumb: Option<InputFile>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration: Option<i32>,
    pub supports_streaming: Option<bool>
}