use crate::types::{Animation, Message, MessageEntity, PhotoSize};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Game {
    /// Title of the game
    pub title: String,
    /// Description of the game
    pub description: String,
    /// Photo that will be displayed in the game message in chats.
    pub photo: Vec<PhotoSize>,
    /// Optional. Brief description of the game or high scores included in the game message. Can be automatically edited to include current high scores for the game when the bot calls setGameScore, or manually edited using editMessageText. 0-4096 characters.
    pub text: Option<String>,
    /// Optional. Special entities that appear in text, such as usernames, URLs, bot commands, etc.
    pub text_entities: Option<Vec<MessageEntity>>,
    /// Optional. Animation that will be displayed in the game message in chats. Upload via BotFather
    pub animation: Option<Animation>,
}
