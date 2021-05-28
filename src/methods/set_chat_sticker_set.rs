use crate::types::{Chat, Sticker, StickerSet};
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct SetChatStickerSet {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: i32,
    /// Name of the sticker set to be set as the group sticker set
    pub sticker_set_name: String,
}

impl SetChatStickerSet {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "SetChatStickerSet")
    }
}
