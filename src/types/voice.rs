use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Voice{
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: i32,
    pub mime_type: Option<String>,
    pub file_size: Option<i32>
}