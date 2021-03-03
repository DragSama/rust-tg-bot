use serde_json;

#[derive(Debug, Serialize)]
pub struct getUpdates{
    pub offset: Option<i32>,
    pub limit: Option<i32>,
    pub timeout: Option<i32>,
    pub allowed_updates: Option<Vec<String>>
}