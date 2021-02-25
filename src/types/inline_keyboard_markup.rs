use crate::types::{inline_keyboard_button::InlineKeyboardButton, keyboard_button::KeyboardButton}

#[derive(Debug, Deserialize)]
pub struct InlineKeyboardMarkup{
    pub inline_keyboard: Vec<Array> of InlineKeyboardButton
}