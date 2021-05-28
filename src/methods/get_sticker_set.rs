use crate::types::{Sticker, StickerSet};
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct GetStickerSet {
    /// Name of the sticker set
    pub name: String,
}

impl GetStickerSet {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "GetStickerSet")
    }
}
