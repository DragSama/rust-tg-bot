use crate::types::{Chat};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct getChat{
    pub chat_id: i32
}