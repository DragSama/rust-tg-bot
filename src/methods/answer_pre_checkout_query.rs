use crate::types::PreCheckoutQuery;
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "AnswerPreCheckoutQuery does nothing until you `send` it"]
#[derive(Serialize)]
pub struct AnswerPreCheckoutQuery<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the query to be answered
    pub pre_checkout_query_id: String,
    /// Specify True if everything is alright (goods are available, etc.) and the bot is ready to proceed with the order. Use False if there are any problems.
    pub ok: bool,
    /// Required if ok is False. Error message in human readable form that explains the reason for failure to proceed with the checkout (e.g. "Sorry, somebody just bought the last of our amazing black T-shirts while you were busy filling out your payment details. Please choose a different color or garment!"). Telegram will display this message to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl<'a> AnswerPreCheckoutQuery<'a> {
    pub fn new(bot: &'a Bot, pre_checkout_query_id: String, ok: bool) -> Self {
        Self {
            pre_checkout_query_id: pre_checkout_query_id,
            ok: ok,
            error_message: None,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<bool> {
        let string = serde_json::to_string(&self)?;
        let resp = self
            .bot
            .send("answerPreCheckoutQuery", Some(string))
            .await?;
        Ok(serde_json::from_str::<bool>(&resp.text().await?)?)
    }
    pub fn pre_checkout_query_id(mut self, pre_checkout_query_id: String) -> Self {
        self.pre_checkout_query_id = pre_checkout_query_id;
        self
    }
    pub fn ok(mut self, ok: bool) -> Self {
        self.ok = ok;
        self
    }
    pub fn error_message(mut self, error_message: String) -> Self {
        self.error_message = Some(error_message);
        self
    }
}
