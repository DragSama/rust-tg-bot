use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct BotCommand{
    /// Text of the command, 1-32 characters. Can contain only lowercase English letters, digits and underscores.
    pub command: String,
    /// Description of the command, 3-256 characters.
    pub description: String
}