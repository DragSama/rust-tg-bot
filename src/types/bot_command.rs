use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BotCommand{
    pub command: String,
    pub description: String
}