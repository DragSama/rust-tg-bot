use serde_json;
#[derive(Debug, Serialize)]
pub struct setStickerSetThumb{
    pub name: String,
    pub user_id: i64,
    pub thumb: Option<InputFile>
}