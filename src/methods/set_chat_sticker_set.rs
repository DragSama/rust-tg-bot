use crate::types::{Chat, Sticker, StickerSet};
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "SetChatStickerSet does nothing until you `send` it"]
#[derive(Serialize)]
pub struct SetChatStickerSet<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: i32,
    /// Name of the sticker set to be set as the group sticker set
    pub sticker_set_name: String,
}

impl<'a> SetChatStickerSet<'a> {
    pub fn new(bot: &'a Bot, chat_id: i32, sticker_set_name: String) -> Self {
        Self {
            chat_id: chat_id,
            sticker_set_name: sticker_set_name,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<bool> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("setChatStickerSet", Some(string)).await?;
        Ok(serde_json::from_str::<bool>(&resp.text().await?)?)
    }
    pub fn chat_id(mut self, chat_id: i32) -> Self {
        self.chat_id = chat_id;
        self
    }
    pub fn sticker_set_name(mut self, sticker_set_name: String) -> Self {
        self.sticker_set_name = sticker_set_name;
        self
    }
}
