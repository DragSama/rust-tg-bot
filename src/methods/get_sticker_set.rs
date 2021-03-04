use crate::types::{Sticker, StickerSet};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct getStickerSet{
    pub name: String
}