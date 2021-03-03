use crate::types::{photo_size::PhotoSize}

#[derive(Debug, Deserialize)]
pub struct UserProfilePhotos{
    pub total_count: i32,
    pub photos: Vec<Array> of PhotoSize
}