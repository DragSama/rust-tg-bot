use crate::types::{File, InputFile, Sticker};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct uploadStickerFile{
    pub user_id: i32,
    pub png_sticker: InputFile
}