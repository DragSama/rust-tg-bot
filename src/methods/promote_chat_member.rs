use crate::types::{Chat, ChatMember};
use serde::Serialize;
use serde_json;


#[derive(Debug, Serialize)]
pub struct PromoteChatMember{
     /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i32,
     /// Unique identifier of the target user
    pub user_id: i32,
     /// Pass True, if the administrator's presence in the chat is hidden
    pub is_anonymous: Option<bool>,
     /// Pass True, if the administrator can change chat title, photo and other settings
    pub can_change_info: Option<bool>,
     /// Pass True, if the administrator can create channel posts, channels only
    pub can_post_messages: Option<bool>,
     /// Pass True, if the administrator can edit messages of other users and can pin messages, channels only
    pub can_edit_messages: Option<bool>,
     /// Pass True, if the administrator can delete messages of other users
    pub can_delete_messages: Option<bool>,
     /// Pass True, if the administrator can invite new users to the chat
    pub can_invite_users: Option<bool>,
     /// Pass True, if the administrator can restrict, ban or unban chat members
    pub can_restrict_members: Option<bool>,
     /// Pass True, if the administrator can pin messages, supergroups only
    pub can_pin_messages: Option<bool>,
     /// Pass True, if the administrator can add new administrators with a subset of their own privileges or demote administrators that he has promoted, directly or indirectly (promoted by administrators that were appointed by him)
    pub can_promote_members: Option<bool>
}

impl PromoteChatMember {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "PromoteChatMember")
}}