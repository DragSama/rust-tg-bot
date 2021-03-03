use crate::types::{user::User}

#[derive(Debug, Deserialize)]
pub struct ProximityAlertTriggered{
    pub traveler: User,
    pub watcher: User,
    pub distance: i32
}