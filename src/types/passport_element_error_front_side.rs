#[derive(Debug, Serialize)]
pub struct PassportElementErrorFrontSide{
    pub source: String,
    pub type: String,
    pub file_hash: String,
    pub message: String
}