use crate::types::{File, InputFile, Sticker, User};
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "UploadStickerFile does nothing until you `send` it"]
#[derive(Serialize)]
pub struct UploadStickerFile<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// User identifier of sticker file owner
    pub user_id: i32,
    /// PNG image with the sticker, must be up to 512 kilobytes in size, dimensions must not exceed 512px, and either width or height must be exactly 512px. More info on Sending Files
    pub png_sticker: InputFile,
}

impl<'a> UploadStickerFile<'a> {
    pub fn new(bot: &'a Bot, user_id: i32, png_sticker: InputFile) -> Self {
        Self {
            user_id: user_id,
            png_sticker: png_sticker,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<File> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("uploadStickerFile", Some(string)).await?;
        Ok(serde_json::from_str::<File>(&resp.text().await?)?)
    }
    pub fn user_id(mut self, user_id: i32) -> Self {
        self.user_id = user_id;
        self
    }
    pub fn png_sticker(mut self, png_sticker: InputFile) -> Self {
        self.png_sticker = png_sticker;
        self
    }
}
