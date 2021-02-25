#[derive(Debug, Deserialize)]
pub struct PhotoSize{
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i64,
    pub height: i64,
    pub file_size: Option<i64>
}