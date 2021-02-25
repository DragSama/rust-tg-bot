use crate::types::{photo_size::PhotoSize}

#[derive(Debug, Deserialize)]
pub struct StickerSet{
    pub name: String,
    pub title: String,
    pub is_animated: bool,
    pub contains_masks: bool,
    pub stickers: Vec<Sticker>,
    pub thumb: Option<PhotoSize>
}