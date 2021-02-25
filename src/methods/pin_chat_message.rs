use serde_json;
#[derive(Debug, Serialize)]
pub struct pinChatMessage{
    pub chat_id: i64,
    pub message_id: i64,
    pub disable_notification: Option<bool>
}