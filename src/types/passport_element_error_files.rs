use crate::types::{File, PassportElementError, PassportElementErrorFile};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PassportElementErrorFiles {
    /// Error source, must be files
    pub source: String,
    /// The section of the user's Telegram Passport which has the issue, one of "utility_bill", "bank_statement", "rental_agreement", "passport_registration", "temporary_registration"
    pub r#type: String,
    /// List of base64-encoded file hashes
    pub file_hashes: Vec<String>,
    /// Error message
    pub message: String,
}
