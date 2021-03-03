use crate::types::{User};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GameHighScore{
    pub position: i32,
    pub user: User,
    pub score: i32
}