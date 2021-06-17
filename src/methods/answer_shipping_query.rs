use crate::types::{ShippingOption, ShippingQuery};
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "AnswerShippingQuery does nothing until you `send` it"]
#[derive(Serialize)]
pub struct AnswerShippingQuery<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the query to be answered
    pub shipping_query_id: String,
    /// Specify True if delivery to the specified address is possible and False if there are any problems (for example, if delivery to the specified address is not possible)
    pub ok: bool,
    /// Required if ok is True. A JSON-serialized array of available shipping options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_options: Option<Vec<ShippingOption>>,
    /// Required if ok is False. Error message in human readable form that explains why it is impossible to complete the order (e.g. "Sorry, delivery to your desired address is unavailable'). Telegram will display this message to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl<'a> AnswerShippingQuery<'a> {
    pub fn new(bot: &'a Bot, shipping_query_id: String, ok: bool) -> Self {
        Self {
            shipping_query_id: shipping_query_id,
            ok: ok,
            shipping_options: None,
            error_message: None,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<bool> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("answerShippingQuery", Some(string)).await?;
        Ok(serde_json::from_str::<bool>(&resp.text().await?)?)
    }
    pub fn shipping_query_id(mut self, shipping_query_id: String) -> Self {
        self.shipping_query_id = shipping_query_id;
        self
    }
    pub fn ok(mut self, ok: bool) -> Self {
        self.ok = ok;
        self
    }
    pub fn shipping_options(mut self, shipping_options: Vec<ShippingOption>) -> Self {
        self.shipping_options = Some(shipping_options);
        self
    }
    pub fn error_message(mut self, error_message: String) -> Self {
        self.error_message = Some(error_message);
        self
    }
}
