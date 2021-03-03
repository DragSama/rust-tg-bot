use serde_json;

#[derive(Debug, Serialize)]
pub struct addStickerToSet{
    pub user_id: i32,
    pub name: String,
    pub png_sticker: Option<InputFile>,
    pub tgs_sticker: Option<InputFile>,
    pub emojis: String,
    pub mask_position: Option<MaskPosition>
}