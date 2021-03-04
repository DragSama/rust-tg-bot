

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PassportElementErrorUnspecified{
    pub source: String,
    pub r#type: String,
    pub element_hash: String,
    pub message: String
}