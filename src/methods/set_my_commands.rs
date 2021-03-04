use crate::types::{BotCommand};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct setMyCommands{
    pub commands: Vec<BotCommand>
}