use crate::types::{Chat, Sticker, StickerSet};
use serde::Serialize;
use serde_json;


#[derive(Debug, Serialize)]
pub struct DeleteChatStickerSet{
     /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: i32
}

impl DeleteChatStickerSet {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "DeleteChatStickerSet")
}}