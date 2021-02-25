use serde_json;
#[derive(Debug, Serialize)]
pub struct setGameScore{
    pub user_id: i64,
    pub score: i64,
    pub force: Option<bool>,
    pub disable_edit_message: Option<bool>,
    pub chat_id: Option<i64>,
    pub message_id: Option<i64>,
    pub inline_message_id: Option<String>
}