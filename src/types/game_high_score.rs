#[derive(Debug, Serialize)]
pub struct GameHighScore{
    pub position: i64,
    pub user: User,
    pub score: i64
}