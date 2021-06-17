use crate::types::{Chat, Voice};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VoiceChatStarted {}
