use serde_json;
#[derive(Debug, Serialize)]
pub struct getChatMember{
    pub chat_id: i64,
    pub user_id: i64
}