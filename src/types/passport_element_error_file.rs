#[derive(Debug, Serialize)]
pub struct PassportElementErrorFile{
    pub source: String,
    pub type: String,
    pub file_hash: String,
    pub message: String
}