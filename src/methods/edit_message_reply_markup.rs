use serde_json;
#[derive(Debug, Serialize)]
pub struct editMessageReplyMarkup{
    pub chat_id: Option<i64>,
    pub message_id: Option<i64>,
    pub inline_message_id: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>
}