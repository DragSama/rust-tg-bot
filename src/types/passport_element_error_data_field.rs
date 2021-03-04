

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PassportElementErrorDataField{
    pub source: String,
    pub r#type: String,
    pub field_name: String,
    pub data_hash: String,
    pub message: String
}