use serde_json;

#[derive(Debug, Serialize)]
pub struct setGameScore{
    pub user_id: i32,
    pub score: i32,
    pub force: Option<bool>,
    pub disable_edit_message: Option<bool>,
    pub chat_id: Option<i32>,
    pub message_id: Option<i32>,
    pub inline_message_id: Option<String>
}