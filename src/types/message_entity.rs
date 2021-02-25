use crate::types::{user::User}

#[derive(Debug, Serialize)]
pub struct MessageEntity{
    pub type: String,
    pub offset: i64,
    pub length: i64,
    pub url: Option<String>,
    pub user: Option<User>,
    pub language: Option<String>
}