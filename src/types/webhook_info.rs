use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WebhookInfo{
    pub url: String,
    pub has_custom_certificate: bool,
    pub pending_update_count: i32,
    pub ip_address: Option<String>,
    pub last_error_date: Option<i32>,
    pub last_error_message: Option<String>,
    pub max_connections: Option<i32>,
    pub allowed_updates: Option<Vec<String>>
}