use crate::types::{File, Sticker};
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "SetStickerPositionInSet does nothing until you `send` it"]
#[derive(Serialize)]
pub struct SetStickerPositionInSet<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// File identifier of the sticker
    pub sticker: String,
    /// New sticker position in the set, zero-based
    pub position: i32,
}

impl<'a> SetStickerPositionInSet<'a> {
    pub fn new(bot: &'a Bot, sticker: String, position: i32) -> Self {
        Self {
            sticker: sticker,
            position: position,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<bool> {
        let string = serde_json::to_string(&self)?;
        let resp = self
            .bot
            .send("setStickerPositionInSet", Some(string))
            .await?;
        Ok(serde_json::from_str::<bool>(&resp.text().await?)?)
    }
    pub fn sticker(mut self, sticker: String) -> Self {
        self.sticker = sticker;
        self
    }
    pub fn position(mut self, position: i32) -> Self {
        self.position = position;
        self
    }
}
