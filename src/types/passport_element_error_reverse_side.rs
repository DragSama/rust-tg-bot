

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PassportElementErrorReverseSide{
    pub source: String,
    pub r#type: String,
    pub file_hash: String,
    pub message: String
}