use serde_json;

#[derive(Debug, Serialize)]
pub struct setWebhook{
    pub url: String,
    pub certificate: Option<InputFile>,
    pub ip_address: Option<String>,
    pub max_connections: Option<i32>,
    pub allowed_updates: Option<Vec<String>>,
    pub drop_pending_updates: Option<bool>
}