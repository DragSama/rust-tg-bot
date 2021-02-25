use crate::types::{encrypted_credentials::EncryptedCredentials, encrypted_passport_element::EncryptedPassportElement}

#[derive(Debug, Deserialize)]
pub struct PassportData{
    pub data: Vec<EncryptedPassportElement>,
    pub credentials: EncryptedCredentials
}