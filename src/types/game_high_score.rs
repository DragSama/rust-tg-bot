use crate::types::{Game, User};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GameHighScore {
    /// Position in high score table for the game
    pub position: i32,
    /// User
    pub user: User,
    /// Score
    pub score: i32,
}
