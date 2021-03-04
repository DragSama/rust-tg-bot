use crate::types::{File};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct getFile{
    pub file_id: String
}