use crate::types::{Chat, User, Voice};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VoiceChatParticipantsInvited {
    /// Optional. New members that were invited to the voice chat
    pub users: Option<Vec<User>>,
}
