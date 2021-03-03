use crate::types::{user::User}

#[derive(Debug, Deserialize)]
pub struct GameHighScore{
    pub position: i32,
    pub user: User,
    pub score: i32
}