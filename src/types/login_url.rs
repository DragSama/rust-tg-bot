#[derive(Debug, Deserialize)]
pub struct LoginUrl{
    pub url: String,
    pub forward_text: Option<String>,
    pub bot_username: Option<String>,
    pub request_write_access: Option<bool>
}