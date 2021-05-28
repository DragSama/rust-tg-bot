use crate::types::{
    InlineKeyboardMarkup, InlineQuery, InlineQueryResult, InputMessageContent, Message, Venue,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct InlineQueryResultVenue {
    /// Type of the result, must be venue
    pub r#type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Latitude of the venue location in degrees
    pub latitude: f64,
    /// Longitude of the venue location in degrees
    pub longitude: f64,
    /// Title of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Optional. Foursquare identifier of the venue if known
    pub foursquare_id: Option<String>,
    /// Optional. Foursquare type of the venue, if known. (For example, "arts_entertainment/default", "arts_entertainment/aquarium" or "food/icecream".)
    pub foursquare_type: Option<String>,
    /// Optional. Google Places identifier of the venue
    pub google_place_id: Option<String>,
    /// Optional. Google Places type of the venue. (See supported types.)
    pub google_place_type: Option<String>,
    /// Optional. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the venue
    pub input_message_content: Option<InputMessageContent>,
    /// Optional. Url of the thumbnail for the result
    pub thumb_url: Option<String>,
    /// Optional. Thumbnail width
    pub thumb_width: Option<i32>,
    /// Optional. Thumbnail height
    pub thumb_height: Option<i32>,
}
