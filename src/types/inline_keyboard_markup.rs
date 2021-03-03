use crate::types::InlineKeyboardButton;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InlineKeyboardMarkup{
    pub inline_keyboard: Vec<InlineKeyboardButton>
}