use crate::types::{File, PhotoSize};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Audio{
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Duration of the audio in seconds as defined by sender
    pub duration: i32,
    /// Optional. Performer of the audio as defined by sender or by audio tags
    pub performer: Option<String>,
    /// Optional. Title of the audio as defined by sender or by audio tags
    pub title: Option<String>,
    /// Optional. Original filename as defined by sender
    pub file_name: Option<String>,
    /// Optional. MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// Optional. File size
    pub file_size: Option<i32>,
    /// Optional. Thumbnail of the album cover to which the music file belongs
    pub thumb: Option<PhotoSize>
}