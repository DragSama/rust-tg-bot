#[derive(Debug, Serialize)]
pub struct ChatPhoto{
    pub small_file_id: String,
    pub small_file_unique_id: String,
    pub big_file_id: String,
    pub big_file_unique_id: String
}