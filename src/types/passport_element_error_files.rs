

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PassportElementErrorFiles{
    pub source: String,
    pub r#type: String,
    pub file_hashes: Vec<String>,
    pub message: String
}