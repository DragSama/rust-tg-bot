use serde_json;

#[derive(Debug, Serialize)]
pub struct sendChatAction{
    pub chat_id: i32,
    pub action: String
}