use serde::{Deserialize, Serialize};

pub enum FilePath {
    FileID(i32),
    Url(String)
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum InputFile {

}
