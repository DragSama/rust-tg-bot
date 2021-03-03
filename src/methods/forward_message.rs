use serde_json;

#[derive(Debug, Serialize)]
pub struct forwardMessage{
    pub chat_id: i32,
    pub from_chat_id: i32,
    pub disable_notification: Option<bool>,
    pub message_id: i32
}