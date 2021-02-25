use serde_json;
#[derive(Debug, Serialize)]
pub struct forwardMessage{
    pub chat_id: i64,
    pub from_chat_id: i64,
    pub disable_notification: Option<bool>,
    pub message_id: i64
}