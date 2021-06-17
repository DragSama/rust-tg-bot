use crate::types::{File, InputFile, Sticker, StickerSet, User};
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "SetStickerSetThumb does nothing until you `send` it"]
#[derive(Serialize)]
pub struct SetStickerSetThumb<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Sticker set name
    pub name: String,
    /// User identifier of the sticker set owner
    pub user_id: i32,
    /// A PNG image with the thumbnail, must be up to 128 kilobytes in size and have width and height exactly 100px, or a TGS animation with the thumbnail up to 32 kilobytes in size; see https://core.telegram.org/animated_stickers#technical-requirements for animated sticker technical requirements. Pass a file_id as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More info on Sending Files . Animated sticker set thumbnail can't be uploaded via HTTP URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFile>,
}

impl<'a> SetStickerSetThumb<'a> {
    pub fn new(bot: &'a Bot, name: String, user_id: i32) -> Self {
        Self {
            name: name,
            user_id: user_id,
            thumb: None,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<bool> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("setStickerSetThumb", Some(string)).await?;
        Ok(serde_json::from_str::<bool>(&resp.text().await?)?)
    }
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
    pub fn user_id(mut self, user_id: i32) -> Self {
        self.user_id = user_id;
        self
    }
    pub fn thumb(mut self, thumb: InputFile) -> Self {
        self.thumb = Some(thumb);
        self
    }
}
