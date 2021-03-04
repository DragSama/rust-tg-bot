use crate::types::{Chat};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct setChatTitle{
    pub chat_id: i32,
    pub title: String
}