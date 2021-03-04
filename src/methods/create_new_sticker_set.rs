use crate::types::{File, InputFile, MaskPosition, Sticker, StickerSet};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct createNewStickerSet{
    pub user_id: i32,
    pub name: String,
    pub title: String,
    pub png_sticker: Option<InputFile>,
    pub tgs_sticker: Option<InputFile>,
    pub emojis: String,
    pub contains_masks: Option<bool>,
    pub mask_position: Option<MaskPosition>
}