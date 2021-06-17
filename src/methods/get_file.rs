use crate::types::File;
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "GetFile does nothing until you `send` it"]
#[derive(Serialize)]
pub struct GetFile<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// File identifier to get info about
    pub file_id: String,
}

impl<'a> GetFile<'a> {
    pub fn new(bot: &'a Bot, file_id: String) -> Self {
        Self {
            file_id: file_id,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<File> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("getFile", Some(string)).await?;
        Ok(serde_json::from_str::<File>(&resp.text().await?)?)
    }
    pub fn file_id(mut self, file_id: String) -> Self {
        self.file_id = file_id;
        self
    }
}
