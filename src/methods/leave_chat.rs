use crate::types::{Chat};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct leaveChat{
    pub chat_id: i32
}