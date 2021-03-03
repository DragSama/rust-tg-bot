use crate::types::{Message, MessageEntity, PollOption};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Poll{
    pub id: String,
    pub question: String,
    pub options: Vec<PollOption>,
    pub total_voter_count: i32,
    pub is_closed: bool,
    pub is_anonymous: bool,
    pub type: String,
    pub allows_multiple_answers: bool,
    pub correct_option_id: Option<i32>,
    pub explanation: Option<String>,
    pub explanation_entities: Option<Vec<MessageEntity>>,
    pub open_period: Option<i32>,
    pub close_date: Option<i32>
}