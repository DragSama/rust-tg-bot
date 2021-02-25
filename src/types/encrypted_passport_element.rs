use crate::types::{file::File, passport_file::PassportFile}

#[derive(Debug, Serialize)]
pub struct EncryptedPassportElement{
    pub type: String,
    pub data: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub files: Option<Vec<PassportFile>>,
    pub front_side: Option<PassportFile>,
    pub reverse_side: Option<PassportFile>,
    pub selfie: Option<PassportFile>,
    pub translation: Option<Vec<PassportFile>>,
    pub hash: String
}