use crate::types::{File, Sticker};
use serde::Serialize;
use serde_json;


#[derive(Debug, Serialize)]
pub struct SetStickerPositionInSet{
     /// File identifier of the sticker
    pub sticker: String,
     /// New sticker position in the set, zero-based
    pub position: i32
}

impl SetStickerPositionInSet {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "SetStickerPositionInSet")
}}