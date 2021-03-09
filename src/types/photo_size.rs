use crate::types::{File};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PhotoSize{
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Photo width
    pub width: i32,
    /// Photo height
    pub height: i32,
    /// Optional. File size
    pub file_size: Option<i32>
}