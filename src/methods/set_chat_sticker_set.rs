use crate::types::{Chat, Sticker, StickerSet};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct setChatStickerSet{
    pub chat_id: i32,
    pub sticker_set_name: String
}