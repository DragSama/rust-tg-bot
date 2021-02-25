use serde_json;
#[derive(Debug, Serialize)]
pub struct setMyCommands{
    pub commands: Vec<BotCommand>
}