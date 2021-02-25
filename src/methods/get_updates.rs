use serde_json;
#[derive(Debug, Serialize)]
pub struct getUpdates{
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub timeout: Option<i64>,
    pub allowed_updates: Option<Vec<String>>
}