use crate::types::{Chat, Voice};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VoiceChatEnded {
    /// Voice chat duration; in seconds
    pub duration: i32,
}
