use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Optional. File size, if known
    pub file_size: Option<i32>,
    /// Optional. File path. Use https://api.telegram.org/file/bot<token>/<file_path> to get the file.
    pub file_path: Option<String>,
}
