use crate::types::{User, UserProfilePhotos};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct getUserProfilePhotos{
    pub user_id: i32,
    pub offset: Option<i32>,
    pub limit: Option<i32>
}