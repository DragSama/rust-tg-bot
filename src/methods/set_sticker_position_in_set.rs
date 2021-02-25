use serde_json;
#[derive(Debug, Serialize)]
pub struct setStickerPositionInSet{
    pub sticker: String,
    pub position: i64
}