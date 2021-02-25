use serde_json;
#[derive(Debug, Serialize)]
pub struct kickChatMember{
    pub chat_id: i64,
    pub user_id: i64,
    pub until_date: Option<i64>
}