use crate::types::{File, InputFile, MaskPosition, Sticker, User};
use serde::Serialize;
use serde_json;


#[derive(Debug, Serialize)]
pub struct AddStickerToSet{
     /// User identifier of sticker set owner
    pub user_id: i32,
     /// Sticker set name
    pub name: String,
     /// PNG image with the sticker, must be up to 512 kilobytes in size, dimensions must not exceed 512px, and either width or height must be exactly 512px. Pass a file_id as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More info on Sending Files 
    pub png_sticker: Option<InputFile>,
     /// TGS animation with the sticker, uploaded using multipart/form-data. See https://core.telegram.org/animated_stickers#technical-requirements for technical requirements
    pub tgs_sticker: Option<InputFile>,
     /// One or more emoji corresponding to the sticker
    pub emojis: String,
     /// A JSON-serialized object for position where the mask should be placed on faces
    pub mask_position: Option<MaskPosition>
}

impl AddStickerToSet {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "AddStickerToSet")
}}