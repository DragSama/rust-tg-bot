use crate::types::{Chat, User};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatMember {
    /// Information about the user
    pub user: User,
    /// The member's status in the chat. Can be "creator", "administrator", "member", "restricted", "left" or "kicked"
    pub status: String,
    /// Optional. Owner and administrators only. Custom title for this user
    pub custom_title: Option<String>,
    /// Optional. Owner and administrators only. True, if the user's presence in the chat is hidden
    pub is_anonymous: Option<bool>,
    /// Optional. Administrators only. True, if the bot is allowed to edit administrator privileges of that user
    pub can_be_edited: Option<bool>,
    /// Optional. Administrators only. True, if the administrator can access the chat event log, chat statistics, message statistics in channels, see channel members, see anonymous administrators in supergroups and ignore slow mode. Implied by any other administrator privilege
    pub can_manage_chat: Option<bool>,
    /// Optional. Administrators only. True, if the administrator can post in the channel; channels only
    pub can_post_messages: Option<bool>,
    /// Optional. Administrators only. True, if the administrator can edit messages of other users and can pin messages; channels only
    pub can_edit_messages: Option<bool>,
    /// Optional. Administrators only. True, if the administrator can delete messages of other users
    pub can_delete_messages: Option<bool>,
    /// Optional. Administrators only. True, if the administrator can manage voice chats
    pub can_manage_voice_chats: Option<bool>,
    /// Optional. Administrators only. True, if the administrator can restrict, ban or unban chat members
    pub can_restrict_members: Option<bool>,
    /// Optional. Administrators only. True, if the administrator can add new administrators with a subset of their own privileges or demote administrators that he has promoted, directly or indirectly (promoted by administrators that were appointed by the user)
    pub can_promote_members: Option<bool>,
    /// Optional. Administrators and restricted only. True, if the user is allowed to change the chat title, photo and other settings
    pub can_change_info: Option<bool>,
    /// Optional. Administrators and restricted only. True, if the user is allowed to invite new users to the chat
    pub can_invite_users: Option<bool>,
    /// Optional. Administrators and restricted only. True, if the user is allowed to pin messages; groups and supergroups only
    pub can_pin_messages: Option<bool>,
    /// Optional. Restricted only. True, if the user is a member of the chat at the moment of the request
    pub is_member: Option<bool>,
    /// Optional. Restricted only. True, if the user is allowed to send text messages, contacts, locations and venues
    pub can_send_messages: Option<bool>,
    /// Optional. Restricted only. True, if the user is allowed to send audios, documents, photos, videos, video notes and voice notes
    pub can_send_media_messages: Option<bool>,
    /// Optional. Restricted only. True, if the user is allowed to send polls
    pub can_send_polls: Option<bool>,
    /// Optional. Restricted only. True, if the user is allowed to send animations, games, stickers and use inline bots
    pub can_send_other_messages: Option<bool>,
    /// Optional. Restricted only. True, if the user is allowed to add web page previews to their messages
    pub can_add_web_page_previews: Option<bool>,
    /// Optional. Restricted and kicked only. Date when restrictions will be lifted for this user; unix time
    pub until_date: Option<i32>,
}
