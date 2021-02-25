use crate::types::{user::User}

#[derive(Debug, Serialize)]
pub struct ChatMember{
    pub user: User,
    pub status: String,
    pub custom_title: Option<String>,
    pub is_anonymous: Option<bool>,
    pub can_be_edited: Option<bool>,
    pub can_post_messages: Option<bool>,
    pub can_edit_messages: Option<bool>,
    pub can_delete_messages: Option<bool>,
    pub can_restrict_members: Option<bool>,
    pub can_promote_members: Option<bool>,
    pub can_change_info: Option<bool>,
    pub can_invite_users: Option<bool>,
    pub can_pin_messages: Option<bool>,
    pub is_member: Option<bool>,
    pub can_send_messages: Option<bool>,
    pub can_send_media_messages: Option<bool>,
    pub can_send_polls: Option<bool>,
    pub can_send_other_messages: Option<bool>,
    pub can_add_web_page_previews: Option<bool>,
    pub until_date: Option<i64>
}