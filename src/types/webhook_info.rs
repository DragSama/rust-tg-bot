use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct WebhookInfo {
    /// Webhook URL, may be empty if webhook is not set up
    pub url: String,
    /// True, if a custom certificate was provided for webhook certificate checks
    pub has_custom_certificate: bool,
    /// Number of updates awaiting delivery
    pub pending_update_count: i32,
    /// Optional. Currently used webhook IP address
    pub ip_address: Option<String>,
    /// Optional. Unix time for the most recent error that happened when trying to deliver an update via webhook
    pub last_error_date: Option<i32>,
    /// Optional. Error message in human-readable format for the most recent error that happened when trying to deliver an update via webhook
    pub last_error_message: Option<String>,
    /// Optional. Maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery
    pub max_connections: Option<i32>,
    /// Optional. A list of update types the bot is subscribed to. Defaults to all update types
    pub allowed_updates: Option<Vec<String>>,
}
