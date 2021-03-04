use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct deleteWebhook{
    pub drop_pending_updates: Option<bool>
}