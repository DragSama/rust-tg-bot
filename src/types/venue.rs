use crate::types::{Location};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Venue{
    pub location: Location,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
    pub foursquare_type: Option<String>,
    pub google_place_id: Option<String>,
    pub google_place_type: Option<String>
}