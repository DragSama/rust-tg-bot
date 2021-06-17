use crate::types::{PassportData, PassportElementError, User};
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "SetPassportDataErrors does nothing until you `send` it"]
#[derive(Serialize)]
pub struct SetPassportDataErrors<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// User identifier
    pub user_id: i32,
    /// A JSON-serialized array describing the errors
    pub errors: Vec<PassportElementError>,
}

impl<'a> SetPassportDataErrors<'a> {
    pub fn new(bot: &'a Bot, user_id: i32, errors: Vec<PassportElementError>) -> Self {
        Self {
            user_id: user_id,
            errors: errors,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<bool> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("setPassportDataErrors", Some(string)).await?;
        Ok(serde_json::from_str::<bool>(&resp.text().await?)?)
    }
    pub fn user_id(mut self, user_id: i32) -> Self {
        self.user_id = user_id;
        self
    }
    pub fn errors(mut self, errors: Vec<PassportElementError>) -> Self {
        self.errors = errors;
        self
    }
}
