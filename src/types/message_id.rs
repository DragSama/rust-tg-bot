use crate::types::Message;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MessageId {
    /// Unique message identifier
    pub message_id: i32,
}
