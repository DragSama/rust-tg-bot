use crate::types::{File, MaskPosition, PhotoSize};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Sticker {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Sticker width
    pub width: i32,
    /// Sticker height
    pub height: i32,
    /// True, if the sticker is animated
    pub is_animated: bool,
    /// Optional. Sticker thumbnail in the .WEBP or .JPG format
    pub thumb: Option<PhotoSize>,
    /// Optional. Emoji associated with the sticker
    pub emoji: Option<String>,
    /// Optional. Name of the sticker set to which the sticker belongs
    pub set_name: Option<String>,
    /// Optional. For mask stickers, the position where the mask should be placed
    pub mask_position: Option<MaskPosition>,
    /// Optional. File size
    pub file_size: Option<i32>,
}
