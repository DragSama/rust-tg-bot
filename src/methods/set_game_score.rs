use crate::types::{Game, Message, User};
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "SetGameScore does nothing until you `send` it"]
#[derive(Serialize)]
pub struct SetGameScore<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// User identifier
    pub user_id: i32,
    /// New score, must be non-negative
    pub score: i32,
    /// Pass True, if the high score is allowed to decrease. This can be useful when fixing mistakes or banning cheaters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// Pass True, if the game message should not be automatically edited to include the current scoreboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_edit_message: Option<bool>,
    /// Required if inline_message_id is not specified. Unique identifier for the target chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// Required if inline_message_id is not specified. Identifier of the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}

impl<'a> SetGameScore<'a> {
    pub fn new(bot: &'a Bot, user_id: i32, score: i32) -> Self {
        Self {
            user_id: user_id,
            score: score,
            force: None,
            disable_edit_message: None,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<Message> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("setGameScore", Some(string)).await?;
        Ok(serde_json::from_str::<Message>(&resp.text().await?)?)
    }
    pub fn user_id(mut self, user_id: i32) -> Self {
        self.user_id = user_id;
        self
    }
    pub fn score(mut self, score: i32) -> Self {
        self.score = score;
        self
    }
    pub fn force(mut self, force: bool) -> Self {
        self.force = Some(force);
        self
    }
    pub fn disable_edit_message(mut self, disable_edit_message: bool) -> Self {
        self.disable_edit_message = Some(disable_edit_message);
        self
    }
    pub fn chat_id(mut self, chat_id: i64) -> Self {
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
