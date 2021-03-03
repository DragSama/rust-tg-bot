use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EncryptedCredentials{
    pub data: String,
    pub hash: String,
    pub secret: String
}