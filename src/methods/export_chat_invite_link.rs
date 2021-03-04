use crate::types::{Chat};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct exportChatInviteLink{
    pub chat_id: i32
}