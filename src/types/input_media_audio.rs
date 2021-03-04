use crate::types::{Audio, File, InputFile, InputMedia, Message, MessageEntity};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InputMediaAudio{
    pub r#type: String,
    pub media: String,
    pub thumb: Option<InputFile>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub duration: Option<i32>,
    pub performer: Option<String>,
    pub title: Option<String>
}