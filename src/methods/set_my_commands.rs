use crate::types::BotCommand;
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct SetMyCommands {
    /// A JSON-serialized list of bot commands to be set as the list of the bot's commands. At most 100 commands can be specified.
    pub commands: Vec<BotCommand>,
}

impl SetMyCommands {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "SetMyCommands")
    }
}
