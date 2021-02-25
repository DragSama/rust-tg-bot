use serde_json;
#[derive(Debug, Serialize)]
pub struct deleteMessage{
    pub chat_id: i64,
    pub message_id: i64
}