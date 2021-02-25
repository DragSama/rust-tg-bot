use serde_json;
#[derive(Debug, Serialize)]
pub struct setChatDescription{
    pub chat_id: i64,
    pub description: Option<String>
}