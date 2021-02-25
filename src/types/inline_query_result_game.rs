use crate::types::{inline_keyboard_markup::InlineKeyboardMarkup}

#[derive(Debug, Deserialize)]
pub struct InlineQueryResultGame{
    pub type: String,
    pub id: String,
    pub game_short_name: String,
    pub reply_markup: Option<InlineKeyboardMarkup>
}