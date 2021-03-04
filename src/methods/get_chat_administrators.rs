use crate::types::{Chat};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct getChatAdministrators{
    pub chat_id: i32
}