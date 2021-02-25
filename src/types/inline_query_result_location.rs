use crate::types::{inline_keyboard_markup::InlineKeyboardMarkup, message::Message}

#[derive(Debug, Deserialize)]
pub struct InlineQueryResultLocation{
    pub type: String,
    pub id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub horizontal_accuracy: Option<f64>,
    pub live_period: Option<i64>,
    pub heading: Option<i64>,
    pub proximity_alert_radius: Option<i64>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
    pub thumb_url: Option<String>,
    pub thumb_width: Option<i64>,
    pub thumb_height: Option<i64>
}