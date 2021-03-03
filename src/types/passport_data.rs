use crate::types::{EncryptedCredentials, EncryptedPassportElement};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PassportData{
    pub data: Vec<EncryptedPassportElement>,
    pub credentials: EncryptedCredentials
}