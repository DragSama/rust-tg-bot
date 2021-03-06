use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum InputFile {
    Url(String),
    File(Vec<u8>)
}
