use crate::types::{InlineKeyboardButton, KeyboardButton};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct InlineKeyboardMarkup{
    /// Vec<button> rows, each represented by an Vec<InlineKeyboardButton> objects
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>
}