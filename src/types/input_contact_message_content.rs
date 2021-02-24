#[derive(Debug, Serialize)]
pub struct InputContactMessageContent{
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub vcard: Option<String>
}