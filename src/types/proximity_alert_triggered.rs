use crate::types::{User};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ProximityAlertTriggered{
    pub traveler: User,
    pub watcher: User,
    pub distance: i32
}