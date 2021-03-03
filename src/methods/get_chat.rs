use serde_json;

#[derive(Debug, Serialize)]
pub struct getChat{
    pub chat_id: i32
}