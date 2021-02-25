use crate::types::{location::Location, user::User}

#[derive(Debug, Deserialize)]
pub struct ChosenInlineResult{
    pub result_id: String,
    pub from: User,
    pub location: Option<Location>,
    pub inline_message_id: Option<String>,
    pub query: String
}