use crate::types::{Chat};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct setChatAdministratorCustomTitle{
    pub chat_id: i32,
    pub user_id: i32,
    pub custom_title: String
}