use crate::types::{InlineKeyboardMarkup, User, Venue};
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct SendVenue {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i32,
    /// Latitude of the venue
    pub latitude: f64,
    /// Longitude of the venue
    pub longitude: f64,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue, if known. (For example, "arts_entertainment/default", "arts_entertainment/aquarium" or "food/icecream".)
    pub foursquare_type: Option<String>,
    /// Google Places identifier of the venue
    pub google_place_id: Option<String>,
    /// Google Places type of the venue. (See supported types.)
    pub google_place_type: Option<String>,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i32>,
    /// Pass True, if the message should be sent even if the specified replied-to message is not found
    pub allow_sending_without_reply: Option<bool>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl SendVenue {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "SendVenue")
    }
}
