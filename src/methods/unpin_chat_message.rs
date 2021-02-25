use serde_json;
#[derive(Debug, Serialize)]
pub struct unpinChatMessage{
    pub chat_id: i64,
    pub message_id: Option<i64>
}