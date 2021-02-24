#[derive(Debug, Serialize)]
pub struct InputMediaAnimation{
    pub type: String,
    pub media: String,
    pub thumb: Option<InputFile>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub duration: Option<i64>
}