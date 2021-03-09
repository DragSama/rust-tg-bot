use crate::types::{File, PassportElementError};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PassportElementErrorTranslationFile{
    /// Error source, must be translation_file
    pub source: String,
    /// Type of element of the user's Telegram Passport which has the issue, one of "passport", "driver_license", "identity_card", "internal_passport", "utility_bill", "bank_statement", "rental_agreement", "passport_registration", "temporary_registration"
    pub r#type: String,
    /// Base64-encoded file hash
    pub file_hash: String,
    /// Error message
    pub message: String
}