use serde_json;
#[derive(Debug, Serialize)]
pub struct sendPoll{
    pub chat_id: i64,
    pub question: String,
    pub options: Vec<String>,
    pub is_anonymous: Option<bool>,
    pub type: Option<String>,
    pub allows_multiple_answers: Option<bool>,
    pub correct_option_id: Option<i64>,
    pub explanation: Option<String>,
    pub explanation_parse_mode: Option<String>,
    pub explanation_entities: Option<Vec<MessageEntity>>,
    pub open_period: Option<i64>,
    pub close_date: Option<i64>,
    pub is_closed: Option<bool>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub allow_sending_without_reply: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>
}