use crate::types::{location::Location, user::User}

#[derive(Debug, Serialize)]
pub struct InlineQuery{
    pub id: String,
    pub from: User,
    pub location: Option<Location>,
    pub query: String,
    pub offset: String
}