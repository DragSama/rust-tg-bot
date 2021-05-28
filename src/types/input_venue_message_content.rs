use crate::types::{Message, Venue};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct InputVenueMessageContent {
    /// Latitude of the venue in degrees
    pub latitude: f64,
    /// Longitude of the venue in degrees
    pub longitude: f64,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Optional. Foursquare identifier of the venue, if known
    pub foursquare_id: Option<String>,
    /// Optional. Foursquare type of the venue, if known. (For example, "arts_entertainment/default", "arts_entertainment/aquarium" or "food/icecream".)
    pub foursquare_type: Option<String>,
    /// Optional. Google Places identifier of the venue
    pub google_place_id: Option<String>,
    /// Optional. Google Places type of the venue. (See supported types.)
    pub google_place_type: Option<String>,
}
