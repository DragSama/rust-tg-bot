use crate::types::{InlineKeyboardMarkup, Message, MessageEntity, Poll};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct sendPoll{
    pub chat_id: i32,
    pub question: String,
    pub options: Vec<String>,
    pub is_anonymous: Option<bool>,
    pub r#type: Option<String>,
    pub allows_multiple_answers: Option<bool>,
    pub correct_option_id: Option<i32>,
    pub explanation: Option<String>,
    pub explanation_parse_mode: Option<String>,
    pub explanation_entities: Option<Vec<MessageEntity>>,
    pub open_period: Option<i32>,
    pub close_date: Option<i32>,
    pub is_closed: Option<bool>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i32>,
    pub allow_sending_without_reply: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>
}