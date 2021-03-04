

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PassportFile{
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: i32,
    pub file_date: i32
}