#[derive(Debug, Serialize)]
pub struct ResponseParameters{
    pub migrate_to_chat_id: Option<i64>,
    pub retry_after: Option<i64>
}