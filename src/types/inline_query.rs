use crate::types::{Location, User};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InlineQuery{
    pub id: String,
    pub from: User,
    pub location: Option<Location>,
    pub query: String,
    pub offset: String
}