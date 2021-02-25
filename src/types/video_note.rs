use crate::types::{photo_size::PhotoSize}

#[derive(Debug, Deserialize)]
pub struct VideoNote{
    pub file_id: String,
    pub file_unique_id: String,
    pub length: i64,
    pub duration: i64,
    pub thumb: Option<PhotoSize>,
    pub file_size: Option<i64>
}