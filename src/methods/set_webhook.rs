use crate::types::{File, InputFile, Update};
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "SetWebhook does nothing until you `send` it"]
#[derive(Serialize)]
pub struct SetWebhook<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// HTTPS url to send updates to. Use an empty string to remove webhook integration
    pub url: String,
    /// Upload your public key certificate so that the root certificate in use can be checked. See our self-signed guide for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<InputFile>,
    /// The fixed IP address which will be used to send webhook requests instead of the IP address resolved through DNS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// Maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery, 1-100. Defaults to 40. Use lower values to limit the load on your bot's server, and higher values to increase your bot's throughput.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i32>,
    /// A JSON-serialized list of the update types you want your bot to receive. For example, specify ["message", "edited_channel_post", "callback_query"] to only receive updates of these types. See Update for a complete list of available update types. Specify an empty list to receive all update types except chat_member (default). If not specified, the previous setting will be used.Please note that this parameter doesn't affect updates created before the call to the setWebhook, so unwanted updates may be received for a short period of time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
    /// Pass True to drop all pending updates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_pending_updates: Option<bool>,
}

impl<'a> SetWebhook<'a> {
    pub fn new(bot: &'a Bot, url: String) -> Self {
        Self {
            url: url,
            certificate: None,
            ip_address: None,
            max_connections: None,
            allowed_updates: None,
            drop_pending_updates: None,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<bool> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("setWebhook", Some(string)).await?;
        Ok(serde_json::from_str::<bool>(&resp.text().await?)?)
    }
    pub fn url(mut self, url: String) -> Self {
        self.url = url;
        self
    }
    pub fn certificate(mut self, certificate: InputFile) -> Self {
        self.certificate = Some(certificate);
        self
    }
    pub fn ip_address(mut self, ip_address: String) -> Self {
        self.ip_address = Some(ip_address);
        self
    }
    pub fn max_connections(mut self, max_connections: i32) -> Self {
        self.max_connections = Some(max_connections);
        self
    }
    pub fn allowed_updates(mut self, allowed_updates: Vec<String>) -> Self {
        self.allowed_updates = Some(allowed_updates);
        self
    }
    pub fn drop_pending_updates(mut self, drop_pending_updates: bool) -> Self {
        self.drop_pending_updates = Some(drop_pending_updates);
        self
    }
}
