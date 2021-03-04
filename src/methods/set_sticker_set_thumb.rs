use crate::types::{File, InputFile, Sticker, StickerSet};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct setStickerSetThumb{
    pub name: String,
    pub user_id: i32,
    pub thumb: Option<InputFile>
}