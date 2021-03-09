use crate::types::{Chat, Location};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatLocation{
    /// The location to which the supergroup is connected. Can't be a live location.
    pub location: Location,
    /// Location address; 1-64 characters, as defined by the chat owner
    pub address: String
}