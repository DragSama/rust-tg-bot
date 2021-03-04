

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PassportElementErrorFile{
    pub source: String,
    pub r#type: String,
    pub file_hash: String,
    pub message: String
}