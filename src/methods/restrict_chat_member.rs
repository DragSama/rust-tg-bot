use serde_json;
#[derive(Debug, Serialize)]
pub struct restrictChatMember{
    pub chat_id: i64,
    pub user_id: i64,
    pub permissions: ChatPermissions,
    pub until_date: Option<i64>
}