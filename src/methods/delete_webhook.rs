use serde::Serialize;
use serde_json;


#[derive(Debug, Serialize)]
pub struct DeleteWebhook{
     /// Pass True to drop all pending updates
    pub drop_pending_updates: Option<bool>
}

impl DeleteWebhook {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "DeleteWebhook")
}}