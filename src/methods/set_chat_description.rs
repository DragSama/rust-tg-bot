use crate::types::{Chat};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct setChatDescription{
    pub chat_id: i32,
    pub description: Option<String>
}