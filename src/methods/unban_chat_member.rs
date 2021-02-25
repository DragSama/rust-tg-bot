use serde_json;
#[derive(Debug, Serialize)]
pub struct unbanChatMember{
    pub chat_id: i64,
    pub user_id: i64,
    pub only_if_banned: Option<bool>
}