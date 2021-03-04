use crate::types::{User};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MessageEntity{
    pub r#type: String,
    pub offset: i32,
    pub length: i32,
    pub url: Option<String>,
    pub user: Option<User>,
    pub language: Option<String>
}