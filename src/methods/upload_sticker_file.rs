use serde_json;
#[derive(Debug, Serialize)]
pub struct uploadStickerFile{
    pub user_id: i64,
    pub png_sticker: InputFile
}