use serde_json;

#[derive(Debug, Serialize)]
pub struct sendAudio{
    pub chat_id: i32,
    pub audio: InputFile,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub duration: Option<i32>,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub thumb: Option<InputFile>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i32>,
    pub allow_sending_without_reply: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>
}