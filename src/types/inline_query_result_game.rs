use crate::types::{InlineKeyboardMarkup};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InlineQueryResultGame{
    pub type: String,
    pub id: String,
    pub game_short_name: String,
    pub reply_markup: Option<InlineKeyboardMarkup>
}