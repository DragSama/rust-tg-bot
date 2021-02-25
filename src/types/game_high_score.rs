use crate::types::{user::User}

#[derive(Debug, Deserialize)]
pub struct GameHighScore{
    pub position: i64,
    pub user: User,
    pub score: i64
}