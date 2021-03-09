use crate::types::{PassportElementError};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PassportElementErrorUnspecified{
    /// Error source, must be unspecified
    pub source: String,
    /// Type of element of the user's Telegram Passport which has the issue
    pub r#type: String,
    /// Base64-encoded element hash
    pub element_hash: String,
    /// Error message
    pub message: String
}