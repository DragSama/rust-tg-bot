use crate::types::{message::Message, message_entity::MessageEntity, poll_option::PollOption}

#[derive(Debug, Deserialize)]
pub struct Poll{
    pub id: String,
    pub question: String,
    pub options: Vec<PollOption>,
    pub total_voter_count: i64,
    pub is_closed: bool,
    pub is_anonymous: bool,
    pub type: String,
    pub allows_multiple_answers: bool,
    pub correct_option_id: Option<i64>,
    pub explanation: Option<String>,
    pub explanation_entities: Option<Vec<MessageEntity>>,
    pub open_period: Option<i64>,
    pub close_date: Option<i64>
}