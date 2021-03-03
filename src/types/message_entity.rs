use crate::types::{user::User}

#[derive(Debug, Deserialize)]
pub struct MessageEntity{
    pub type: String,
    pub offset: i32,
    pub length: i32,
    pub url: Option<String>,
    pub user: Option<User>,
    pub language: Option<String>
}