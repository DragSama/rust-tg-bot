use crate::types::{inline_keyboard_markup::InlineKeyboardMarkup, input_message_content::InputMessageContent, message::Message}

#[derive(Debug, Deserialize)]
pub struct InlineQueryResultLocation{
    pub type: String,
    pub id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub horizontal_accuracy: Option<f64>,
    pub live_period: Option<i32>,
    pub heading: Option<i32>,
    pub proximity_alert_radius: Option<i32>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
    pub thumb_url: Option<String>,
    pub thumb_width: Option<i32>,
    pub thumb_height: Option<i32>
}