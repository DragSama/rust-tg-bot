use crate::types::{Game, Message, User};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CallbackQuery {
    /// Unique identifier for this query
    pub id: String,
    /// Sender
    pub from: User,
    /// Optional. Message with the callback button that originated the query. Note that message content and message date will not be available if the message is too old
    pub message: Option<Message>,
    /// Optional. Identifier of the message sent via the bot in inline mode, that originated the query.
    pub inline_message_id: Option<String>,
    /// Global identifier, uniquely corresponding to the chat to which the message with the callback button was sent. Useful for high scores in games.
    pub chat_instance: String,
    /// Optional. Data associated with the callback button. Be aware that a bad client can send arbitrary data in this field.
    pub data: Option<String>,
    /// Optional. Short name of a Game to be returned, serves as the unique identifier for the game
    pub game_short_name: Option<String>,
}
