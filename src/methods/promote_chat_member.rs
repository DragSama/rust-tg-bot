use serde_json;
#[derive(Debug, Serialize)]
pub struct promoteChatMember{
    pub chat_id: i64,
    pub user_id: i64,
    pub is_anonymous: Option<bool>,
    pub can_change_info: Option<bool>,
    pub can_post_messages: Option<bool>,
    pub can_edit_messages: Option<bool>,
    pub can_delete_messages: Option<bool>,
    pub can_invite_users: Option<bool>,
    pub can_restrict_members: Option<bool>,
    pub can_pin_messages: Option<bool>,
    pub can_promote_members: Option<bool>
}