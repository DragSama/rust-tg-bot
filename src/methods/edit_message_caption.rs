use serde_json;

#[derive(Debug, Serialize)]
pub struct editMessageCaption{
    pub chat_id: Option<i32>,
    pub message_id: Option<i32>,
    pub inline_message_id: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub reply_markup: Option<InlineKeyboardMarkup>
}