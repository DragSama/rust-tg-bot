use serde_json;

#[derive(Debug, Serialize)]
pub struct sendDocument{
    pub chat_id: i32,
    pub document: InputFile,
    pub thumb: Option<InputFile>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub disable_content_type_detection: Option<bool>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i32>,
    pub allow_sending_without_reply: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>
}