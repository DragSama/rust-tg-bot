use serde_json;

#[derive(Debug, Serialize)]
pub struct restrictChatMember{
    pub chat_id: i32,
    pub user_id: i32,
    pub permissions: ChatPermissions,
    pub until_date: Option<i32>
}