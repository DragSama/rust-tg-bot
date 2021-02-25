use serde_json;
#[derive(Debug, Serialize)]
pub struct getGameHighScores{
    pub user_id: i64,
    pub chat_id: Option<i64>,
    pub message_id: Option<i64>,
    pub inline_message_id: Option<String>
}