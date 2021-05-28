use crate::types::{File, InputFile, Sticker, User};
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct UploadStickerFile {
    /// User identifier of sticker file owner
    pub user_id: i32,
    /// PNG image with the sticker, must be up to 512 kilobytes in size, dimensions must not exceed 512px, and either width or height must be exactly 512px. More info on Sending Files
    pub png_sticker: InputFile,
}

impl UploadStickerFile {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "UploadStickerFile")
    }
}
