use serde_json;

#[derive(Debug, Serialize)]
pub struct unbanChatMember{
    pub chat_id: i32,
    pub user_id: i32,
    pub only_if_banned: Option<bool>
}