use crate::types::{File, InputFile, Update};
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct SetWebhook {
    /// HTTPS url to send updates to. Use an empty string to remove webhook integration
    pub url: String,
    /// Upload your public key certificate so that the root certificate in use can be checked. See our self-signed guide for details.
    pub certificate: Option<InputFile>,
    /// The fixed IP address which will be used to send webhook requests instead of the IP address resolved through DNS
    pub ip_address: Option<String>,
    /// Maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery, 1-100. Defaults to 40. Use lower values to limit the load on your bot's server, and higher values to increase your bot's throughput.
    pub max_connections: Option<i32>,
    /// A JSON-serialized list of the update types you want your bot to receive. For example, specify ["message", "edited_channel_post", "callback_query"] to only receive updates of these types. See Update for a complete list of available update types. Specify an empty list to receive all updates regardless of type (default). If not specified, the previous setting will be used.Please note that this parameter doesn't affect updates created before the call to the setWebhook, so unwanted updates may be received for a short period of time.
    pub allowed_updates: Option<Vec<String>>,
    /// Pass True to drop all pending updates
    pub drop_pending_updates: Option<bool>,
}

impl SetWebhook {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "SetWebhook")
    }
}
