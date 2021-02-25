use crate::types::{photo_size::PhotoSize}

#[derive(Debug, Deserialize)]
pub struct Document{
    pub file_id: String,
    pub file_unique_id: String,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>
}