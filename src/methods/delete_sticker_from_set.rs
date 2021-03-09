use crate::types::{File, Sticker};
use serde::Serialize;
use serde_json;


#[derive(Debug, Serialize)]
pub struct DeleteStickerFromSet{
     /// File identifier of the sticker
    pub sticker: String
}

impl DeleteStickerFromSet {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "DeleteStickerFromSet")
}}