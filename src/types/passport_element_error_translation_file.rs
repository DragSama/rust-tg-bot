use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PassportElementErrorTranslationFile{
    pub source: String,
    pub type: String,
    pub file_hash: String,
    pub message: String
}