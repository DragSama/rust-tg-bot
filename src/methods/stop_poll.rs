use serde_json;
#[derive(Debug, Serialize)]
pub struct stopPoll{
    pub chat_id: i64,
    pub message_id: i64,
    pub reply_markup: Option<InlineKeyboardMarkup>
}