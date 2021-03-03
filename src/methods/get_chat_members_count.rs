use serde_json;

#[derive(Debug, Serialize)]
pub struct getChatMembersCount{
    pub chat_id: i32
}