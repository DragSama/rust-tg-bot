#[derive(Debug, Deserialize)]
pub struct File{
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: Option<i64>,
    pub file_path: Option<String>
}