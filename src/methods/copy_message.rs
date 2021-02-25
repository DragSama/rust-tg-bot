use serde_json;
#[derive(Debug, Serialize)]
pub struct copyMessage{
    pub chat_id: i64,
    pub from_chat_id: i64,
    pub message_id: i64,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub allow_sending_without_reply: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>
}