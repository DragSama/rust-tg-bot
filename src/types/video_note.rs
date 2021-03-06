use crate::types::{File, PhotoSize, Video};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VideoNote {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Video width and height (diameter of the video message) as defined by sender
    pub length: i32,
    /// Duration of the video in seconds as defined by sender
    pub duration: i32,
    /// Optional. Video thumbnail
    pub thumb: Option<PhotoSize>,
    /// Optional. File size
    pub file_size: Option<i32>,
}
