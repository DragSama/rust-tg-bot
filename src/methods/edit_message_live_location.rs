use crate::types::{InlineKeyboardMarkup, Location, Message};
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct EditMessageLiveLocation {
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: Option<i32>,
    /// Required if inline_message_id is not specified. Identifier of the message to edit
    pub message_id: Option<i32>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
    /// Latitude of new location
    pub latitude: f64,
    /// Longitude of new location
    pub longitude: f64,
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    pub horizontal_accuracy: Option<f64>,
    /// Direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    pub heading: Option<i32>,
    /// Maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    pub proximity_alert_radius: Option<i32>,
    /// A JSON-serialized object for a new inline keyboard.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageLiveLocation {
    pub fn get_data(&self) -> (String, &str) {
        (
            serde_json::to_string(&self).unwrap(),
            "EditMessageLiveLocation",
        )
    }
}
