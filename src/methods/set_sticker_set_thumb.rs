use crate::types::{File, InputFile, Sticker, StickerSet, User};
use serde::Serialize;
use serde_json;


#[derive(Debug, Serialize)]
pub struct SetStickerSetThumb{
     /// Sticker set name
    pub name: String,
     /// User identifier of the sticker set owner
    pub user_id: i32,
     /// A PNG image with the thumbnail, must be up to 128 kilobytes in size and have width and height exactly 100px, or a TGS animation with the thumbnail up to 32 kilobytes in size; see https://core.telegram.org/animated_stickers#technical-requirements for animated sticker technical requirements. Pass a file_id as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More info on Sending Files . Animated sticker set thumbnail can't be uploaded via HTTP URL.
    pub thumb: Option<InputFile>
}

impl SetStickerSetThumb {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "SetStickerSetThumb")
}}