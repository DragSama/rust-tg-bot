use crate::types::{File, InputFile, MaskPosition, Sticker, User};
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "AddStickerToSet does nothing until you `send` it"]
#[derive(Serialize)]
pub struct AddStickerToSet<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// User identifier of sticker set owner
    pub user_id: i32,
    /// Sticker set name
    pub name: String,
    /// PNG image with the sticker, must be up to 512 kilobytes in size, dimensions must not exceed 512px, and either width or height must be exactly 512px. Pass a file_id as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More info on Sending Files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub png_sticker: Option<InputFile>,
    /// TGS animation with the sticker, uploaded using multipart/form-data. See https://core.telegram.org/animated_stickers#technical-requirements for technical requirements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tgs_sticker: Option<InputFile>,
    /// One or more emoji corresponding to the sticker
    pub emojis: String,
    /// A JSON-serialized object for position where the mask should be placed on faces
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
}

impl<'a> AddStickerToSet<'a> {
    pub fn new(bot: &'a Bot, user_id: i32, name: String, emojis: String) -> Self {
        Self {
            user_id: user_id,
            name: name,
            emojis: emojis,
            png_sticker: None,
            tgs_sticker: None,
            mask_position: None,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<bool> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("addStickerToSet", Some(string)).await?;
        Ok(serde_json::from_str::<bool>(&resp.text().await?)?)
    }
    pub fn user_id(mut self, user_id: i32) -> Self {
        self.user_id = user_id;
        self
    }
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
    pub fn png_sticker(mut self, png_sticker: InputFile) -> Self {
        self.png_sticker = Some(png_sticker);
        self
    }
    pub fn tgs_sticker(mut self, tgs_sticker: InputFile) -> Self {
        self.tgs_sticker = Some(tgs_sticker);
        self
    }
    pub fn emojis(mut self, emojis: String) -> Self {
        self.emojis = emojis;
        self
    }
    pub fn mask_position(mut self, mask_position: MaskPosition) -> Self {
        self.mask_position = Some(mask_position);
        self
    }
}
