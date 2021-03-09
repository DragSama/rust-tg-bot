use crate::types::{InlineKeyboardMarkup, InlineQuery, InlineQueryResult, InputMessageContent, Location, Message};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct InlineQueryResultLocation{
    /// Type of the result, must be location
    pub r#type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Location latitude in degrees
    pub latitude: f64,
    /// Location longitude in degrees
    pub longitude: f64,
    /// Location title
    pub title: String,
    /// Optional. The radius of uncertainty for the location, measured in meters; 0-1500
    pub horizontal_accuracy: Option<f64>,
    /// Optional. Period in seconds for which the location can be updated, should be between 60 and 86400.
    pub live_period: Option<i32>,
    /// Optional. For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    pub heading: Option<i32>,
    /// Optional. For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    pub proximity_alert_radius: Option<i32>,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the location
    pub input_message_content: Option<InputMessageContent>,
    /// Optional. Url of the thumbnail for the result
    pub thumb_url: Option<String>,
    /// Optional. Thumbnail width
    pub thumb_width: Option<i32>,
    /// Optional. Thumbnail height
    pub thumb_height: Option<i32>
}