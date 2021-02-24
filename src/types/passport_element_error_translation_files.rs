#[derive(Debug, Serialize)]
pub struct PassportElementErrorTranslationFiles{
    pub source: String,
    pub type: String,
    pub file_hashes: Vec<String>,
    pub message: String
}