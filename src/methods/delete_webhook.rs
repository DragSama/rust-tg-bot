use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "DeleteWebhook does nothing until you `send` it"]
#[derive(Serialize)]
pub struct DeleteWebhook<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Pass True to drop all pending updates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_pending_updates: Option<bool>,
}

impl<'a> DeleteWebhook<'a> {
    pub fn new(bot: &'a Bot) -> Self {
        Self {
            drop_pending_updates: None,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<bool> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("deleteWebhook", Some(string)).await?;
        Ok(serde_json::from_str::<bool>(&resp.text().await?)?)
    }
    pub fn drop_pending_updates(mut self, drop_pending_updates: bool) -> Self {
        self.drop_pending_updates = Some(drop_pending_updates);
        self
    }
}
