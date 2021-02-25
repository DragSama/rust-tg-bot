use serde_json;
#[derive(Debug, Serialize)]
pub struct deleteWebhook{
    pub drop_pending_updates: Option<bool>
}