use crate::types::{File, Sticker};
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "DeleteStickerFromSet does nothing until you `send` it"]
#[derive(Serialize)]
pub struct DeleteStickerFromSet<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// File identifier of the sticker
    pub sticker: String,
}

impl<'a> DeleteStickerFromSet<'a> {
    pub fn new(bot: &'a Bot, sticker: String) -> Self {
        Self {
            sticker: sticker,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<bool> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("deleteStickerFromSet", Some(string)).await?;
        Ok(serde_json::from_str::<bool>(&resp.text().await?)?)
    }
    pub fn sticker(mut self, sticker: String) -> Self {
        self.sticker = sticker;
        self
    }
}
