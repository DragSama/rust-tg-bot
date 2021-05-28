use crate::types::PassportElementError;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PassportElementErrorDataField {
    /// Error source, must be data
    pub source: String,
    /// The section of the user's Telegram Passport which has the error, one of "personal_details", "passport", "driver_license", "identity_card", "internal_passport", "address"
    pub r#type: String,
    /// Name of the data field which has the error
    pub field_name: String,
    /// Base64-encoded data hash
    pub data_hash: String,
    /// Error message
    pub message: String,
}
