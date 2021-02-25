use crate::types::{photo_size::PhotoSize}

#[derive(Debug, Serialize)]
pub struct UserProfilePhotos{
    pub total_count: i64,
    pub photos: Vec<Array> of PhotoSize
}