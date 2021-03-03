use serde_json;

#[derive(Debug, Serialize)]
pub struct deleteStickerFromSet{
    pub sticker: String
}