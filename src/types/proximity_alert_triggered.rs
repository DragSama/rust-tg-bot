use crate::types::{user::User}

#[derive(Debug, Serialize)]
pub struct ProximityAlertTriggered{
    pub traveler: User,
    pub watcher: User,
    pub distance: i64
}