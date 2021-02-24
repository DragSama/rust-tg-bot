#[derive(Debug, Serialize)]
pub struct EncryptedCredentials{
    pub data: String,
    pub hash: String,
    pub secret: String
}