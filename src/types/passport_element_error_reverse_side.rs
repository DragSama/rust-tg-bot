use crate::types::PassportElementError;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PassportElementErrorReverseSide {
    /// Error source, must be reverse_side
    pub source: String,
    /// The section of the user's Telegram Passport which has the issue, one of "driver_license", "identity_card"
    pub r#type: String,
    /// Base64-encoded hash of the file with the reverse side of the document
    pub file_hash: String,
    /// Error message
    pub message: String,
}
