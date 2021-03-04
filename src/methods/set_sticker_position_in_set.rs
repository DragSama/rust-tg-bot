use crate::types::{Sticker};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct setStickerPositionInSet{
    pub sticker: String,
    pub position: i32
}