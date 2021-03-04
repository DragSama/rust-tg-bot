use crate::types::{Chat, Sticker, StickerSet};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct deleteChatStickerSet{
    pub chat_id: i32
}