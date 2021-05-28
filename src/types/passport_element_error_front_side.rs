use crate::types::PassportElementError;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PassportElementErrorFrontSide {
    /// Error source, must be front_side
    pub source: String,
    /// The section of the user's Telegram Passport which has the issue, one of "passport", "driver_license", "identity_card", "internal_passport"
    pub r#type: String,
    /// Base64-encoded hash of the file with the front side of the document
    pub file_hash: String,
    /// Error message
    pub message: String,
}
