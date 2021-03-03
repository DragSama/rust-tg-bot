use serde_json;

#[derive(Debug, Serialize)]
pub struct getChatMember{
    pub chat_id: i32,
    pub user_id: i32
}