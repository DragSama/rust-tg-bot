use crate::types::{Chat, User};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatInviteLink {
    /// The invite link. If the link was created by another chat administrator, then the second part of the link will be replaced with "".
    pub invite_link: String,
    /// Creator of the link
    pub creator: User,
    /// True, if the link is primary
    pub is_primary: bool,
    /// True, if the link is revoked
    pub is_revoked: bool,
    /// Optional. Point in time (Unix timestamp) when the link will expire or has been expired
    pub expire_date: Option<i32>,
    /// Optional. Maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    pub member_limit: Option<i32>,
}
