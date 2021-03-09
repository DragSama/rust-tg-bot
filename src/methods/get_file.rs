use crate::types::{File};
use serde::Serialize;
use serde_json;


#[derive(Debug, Serialize)]
pub struct GetFile{
     /// File identifier to get info about
    pub file_id: String
}

impl GetFile {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "GetFile")
}}