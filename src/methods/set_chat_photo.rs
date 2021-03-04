use crate::types::{Chat, ChatPhoto, File, InputFile};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct setChatPhoto{
    pub chat_id: i32,
    pub photo: InputFile
}