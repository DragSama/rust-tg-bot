use crate::types::{InlineKeyboardMarkup, Location, Message};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct editMessageLiveLocation{
    pub chat_id: Option<i32>,
    pub message_id: Option<i32>,
    pub inline_message_id: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub horizontal_accuracy: Option<f64>,
    pub heading: Option<i32>,
    pub proximity_alert_radius: Option<i32>,
    pub reply_markup: Option<InlineKeyboardMarkup>
}