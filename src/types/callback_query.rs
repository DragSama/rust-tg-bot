use crate::types::{Message, User};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CallbackQuery{
    pub id: String,
    pub from: User,
    pub message: Option<Message>,
    pub inline_message_id: Option<String>,
    pub chat_instance: String,
    pub data: Option<String>,
    pub game_short_name: Option<String>
}