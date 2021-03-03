use crate::types::{PhotoSize};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Video{
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i32,
    pub height: i32,
    pub duration: i32,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i32>
}