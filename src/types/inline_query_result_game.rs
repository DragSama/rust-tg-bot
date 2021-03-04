use crate::types::{Game, InlineKeyboardMarkup, InlineQuery, InlineQueryResult};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InlineQueryResultGame{
    pub r#type: String,
    pub id: String,
    pub game_short_name: String,
    pub reply_markup: Option<InlineKeyboardMarkup>
}