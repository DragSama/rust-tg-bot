use crate::types::{File, PhotoSize};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Video {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Video width as defined by sender
    pub width: i32,
    /// Video height as defined by sender
    pub height: i32,
    /// Duration of the video in seconds as defined by sender
    pub duration: i32,
    /// Optional. Video thumbnail
    pub thumb: Option<PhotoSize>,
    /// Optional. Original filename as defined by sender
    pub file_name: Option<String>,
    /// Optional. Mime type of a file as defined by sender
    pub mime_type: Option<String>,
    /// Optional. File size
    pub file_size: Option<i32>,
}
