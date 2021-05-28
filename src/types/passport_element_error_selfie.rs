use crate::types::PassportElementError;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PassportElementErrorSelfie {
    /// Error source, must be selfie
    pub source: String,
    /// The section of the user's Telegram Passport which has the issue, one of "passport", "driver_license", "identity_card", "internal_passport"
    pub r#type: String,
    /// Base64-encoded hash of the file with the selfie
    pub file_hash: String,
    /// Error message
    pub message: String,
}
