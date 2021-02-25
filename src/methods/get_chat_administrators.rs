use serde_json;
#[derive(Debug, Serialize)]
pub struct getChatAdministrators{
    pub chat_id: i64
}