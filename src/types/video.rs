use crate::types::{photo_size::PhotoSize}

#[derive(Debug, Deserialize)]
pub struct Video{
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i64,
    pub height: i64,
    pub duration: i64,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>
}