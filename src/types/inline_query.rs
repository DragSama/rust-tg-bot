use crate::types::{Location, User};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct InlineQuery{
    /// Unique identifier for this query
    pub id: String,
    /// Sender
    pub from: User,
    /// Optional. Sender location, only for bots that request user location
    pub location: Option<Location>,
    /// Text of the query (up to 256 characters)
    pub query: String,
    /// Offset of the results to be returned, can be controlled by the bot
    pub offset: String
}