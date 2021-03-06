use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Location {
    /// Longitude as defined by sender
    pub longitude: f64,
    /// Latitude as defined by sender
    pub latitude: f64,
    /// Optional. The radius of uncertainty for the location, measured in meters; 0-1500
    pub horizontal_accuracy: Option<f64>,
    /// Optional. Time relative to the message sending date, during which the location can be updated, in seconds. For active live locations only.
    pub live_period: Option<i32>,
    /// Optional. The direction in which user is moving, in degrees; 1-360. For active live locations only.
    pub heading: Option<i32>,
    /// Optional. Maximum distance for proximity alerts about approaching another chat member, in meters. For sent live locations only.
    pub proximity_alert_radius: Option<i32>,
}
