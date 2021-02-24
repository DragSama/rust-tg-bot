#[derive(Debug, Serialize)]
pub struct PassportElementErrorUnspecified{
    pub source: String,
    pub type: String,
    pub element_hash: String,
    pub message: String
}