use serde_json;

#[derive(Debug, Serialize)]
pub struct getGameHighScores{
    pub user_id: i32,
    pub chat_id: Option<i32>,
    pub message_id: Option<i32>,
    pub inline_message_id: Option<String>
}