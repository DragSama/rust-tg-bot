use crate::types::{keyboard_button::KeyboardButton}

#[derive(Debug, Deserialize)]
pub struct ReplyKeyboardMarkup{
    pub keyboard: Vec<Array> of KeyboardButton,
    pub resize_keyboard: Option<bool>,
    pub one_time_keyboard: Option<bool>,
    pub selective: Option<bool>
}