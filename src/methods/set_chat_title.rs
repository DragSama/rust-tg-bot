use serde_json;

#[derive(Debug, Serialize)]
pub struct setChatTitle{
    pub chat_id: i32,
    pub title: String
}