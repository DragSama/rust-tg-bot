use crate::types::{Chat, ChatPermissions};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct setChatPermissions{
    pub chat_id: i32,
    pub permissions: ChatPermissions
}