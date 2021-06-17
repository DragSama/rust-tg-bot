use crate::types::{Game, GameHighScore};
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "GetGameHighScores does nothing until you `send` it"]
#[derive(Serialize)]
pub struct GetGameHighScores<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Target user id
    pub user_id: i32,
    /// Required if inline_message_id is not specified. Unique identifier for the target chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i32>,
    /// Required if inline_message_id is not specified. Identifier of the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}

impl<'a> GetGameHighScores<'a> {
    pub fn new(bot: &'a Bot, user_id: i32) -> Self {
        Self {
            user_id: user_id,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<Vec<GameHighScore>> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("getGameHighScores", Some(string)).await?;
        Ok(serde_json::from_str::<Vec<GameHighScore>>(
            &resp.text().await?,
        )?)
    }
    pub fn user_id(mut self, user_id: i32) -> Self {
        self.user_id = user_id;
        self
    }
    pub fn chat_id(mut self, chat_id: i32) -> Self {
        self.chat_id = Some(chat_id);
        self
    }
    pub fn message_id(mut self, message_id: i32) -> Self {
        self.message_id = Some(message_id);
        self
    }
    pub fn inline_message_id(mut self, inline_message_id: String) -> Self {
        self.inline_message_id = Some(inline_message_id);
        self
    }
}
