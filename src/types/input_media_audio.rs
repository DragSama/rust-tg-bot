#[derive(Debug, Serialize)]
pub struct InputMediaAudio{
    pub type: String,
    pub media: String,
    pub thumb: Option<InputFile>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub duration: Option<i64>,
    pub performer: Option<String>,
    pub title: Option<String>
}