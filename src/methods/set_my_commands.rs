use crate::types::BotCommand;
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "SetMyCommands does nothing until you `send` it"]
#[derive(Serialize)]
pub struct SetMyCommands<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// A JSON-serialized list of bot commands to be set as the list of the bot's commands. At most 100 commands can be specified.
    pub commands: Vec<BotCommand>,
}

impl<'a> SetMyCommands<'a> {
    pub fn new(bot: &'a Bot, commands: Vec<BotCommand>) -> Self {
        Self {
            commands: commands,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<bool> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("setMyCommands", Some(string)).await?;
        Ok(serde_json::from_str::<bool>(&resp.text().await?)?)
    }
    pub fn commands(mut self, commands: Vec<BotCommand>) -> Self {
        self.commands = commands;
        self
    }
}
