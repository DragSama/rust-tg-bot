use crate::types::{PhotoSize, User};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserProfilePhotos{
    /// Total number of profile pictures the target user has
    pub total_count: i32,
    /// Requested profile pictures (in up to 4 sizes each)
    pub photos: Vec<Vec<PhotoSize>>
}