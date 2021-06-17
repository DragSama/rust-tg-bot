use crate::types::{Sticker, StickerSet};
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "GetStickerSet does nothing until you `send` it"]
#[derive(Serialize)]
pub struct GetStickerSet<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Name of the sticker set
    pub name: String,
}

impl<'a> GetStickerSet<'a> {
    pub fn new(bot: &'a Bot, name: String) -> Self {
        Self {
            name: name,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<StickerSet> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("getStickerSet", Some(string)).await?;
        Ok(serde_json::from_str::<StickerSet>(&resp.text().await?)?)
    }
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
}
