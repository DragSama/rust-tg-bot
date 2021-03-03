use crate::types::{PhotoSize};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserProfilePhotos{
    pub total_count: i32,
    pub photos: Vec<PhotoSize>
}
