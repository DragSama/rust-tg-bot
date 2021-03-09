use crate::types::{Game, InlineKeyboardMarkup, InlineQuery, InlineQueryResult};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct InlineQueryResultGame{
    /// Type of the result, must be game
    pub r#type: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// Short name of the game
    pub game_short_name: String,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>
}