use serde_json;
#[derive(Debug, Serialize)]
pub struct setChatStickerSet{
    pub chat_id: i64,
    pub sticker_set_name: String
}