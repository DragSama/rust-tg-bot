use crate::types::{Sticker};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct deleteStickerFromSet{
    pub sticker: String
}