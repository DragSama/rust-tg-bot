use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PassportElementErrorReverseSide{
    pub source: String,
    pub type: String,
    pub file_hash: String,
    pub message: String
}