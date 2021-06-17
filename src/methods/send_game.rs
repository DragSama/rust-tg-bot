use crate::types::{Game, InlineKeyboardMarkup, Message, User};
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "SendGame does nothing until you `send` it"]
#[derive(Serialize)]
pub struct SendGame<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat
    pub chat_id: i32,
    /// Short name of the game, serves as the unique identifier for the game. Set up your games via Botfather.
    pub game_short_name: String,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i32>,
    /// Pass True, if the message should be sent even if the specified replied-to message is not found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    /// A JSON-serialized object for an inline keyboard. If empty, one 'Play game_title' button will be shown. If not empty, the first button must launch the game.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl<'a> SendGame<'a> {
    pub fn new(bot: &'a Bot, chat_id: i32, game_short_name: String) -> Self {
        Self {
            chat_id: chat_id,
            game_short_name: game_short_name,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<Message> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("sendGame", Some(string)).await?;
        Ok(serde_json::from_str::<Message>(&resp.text().await?)?)
    }
    pub fn chat_id(mut self, chat_id: i32) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn game_short_name(mut self, game_short_name: String) -> Self {
        self.game_short_name = game_short_name;
        self
    }
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }
    pub fn reply_to_message_id(mut self, reply_to_message_id: i32) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }
    pub fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }
    pub fn reply_markup(mut self, reply_markup: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }
}
