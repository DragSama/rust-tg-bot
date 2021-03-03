use crate::types::{mask_position::MaskPosition, photo_size::PhotoSize}

#[derive(Debug, Deserialize)]
pub struct Sticker{
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i32,
    pub height: i32,
    pub is_animated: bool,
    pub thumb: Option<PhotoSize>,
    pub emoji: Option<String>,
    pub set_name: Option<String>,
    pub mask_position: Option<MaskPosition>,
    pub file_size: Option<i32>
}