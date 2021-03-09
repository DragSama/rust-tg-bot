use crate::types::{File, InputFile, MaskPosition, Sticker, StickerSet, User};
use serde::Serialize;
use serde_json;


#[derive(Debug, Serialize)]
pub struct CreateNewStickerSet{
     /// User identifier of created sticker set owner
    pub user_id: i32,
     /// Short name of sticker set, to be used in t.me/addstickers/ URLs (e.g., animals). Can contain only english letters, digits and underscores. Must begin with a letter, can't contain consecutive underscores and must end in "_by_<bot username>". <bot_username> is case insensitive. 1-64 characters.
    pub name: String,
     /// Sticker set title, 1-64 characters
    pub title: String,
     /// PNG image with the sticker, must be up to 512 kilobytes in size, dimensions must not exceed 512px, and either width or height must be exactly 512px. Pass a file_id as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More info on Sending Files 
    pub png_sticker: Option<InputFile>,
     /// TGS animation with the sticker, uploaded using multipart/form-data. See https://core.telegram.org/animated_stickers#technical-requirements for technical requirements
    pub tgs_sticker: Option<InputFile>,
     /// One or more emoji corresponding to the sticker
    pub emojis: String,
     /// Pass True, if a set of mask stickers should be created
    pub contains_masks: Option<bool>,
     /// A JSON-serialized object for position where the mask should be placed on faces
    pub mask_position: Option<MaskPosition>
}

impl CreateNewStickerSet {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "CreateNewStickerSet")
}}