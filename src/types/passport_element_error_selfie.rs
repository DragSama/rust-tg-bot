#[derive(Debug, Serialize)]
pub struct PassportElementErrorSelfie{
    pub source: String,
    pub type: String,
    pub file_hash: String,
    pub message: String
}