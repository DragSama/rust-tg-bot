use crate::types::{PreCheckoutQuery};
use serde::Serialize;
use serde_json;


#[derive(Debug, Serialize)]
pub struct AnswerPreCheckoutQuery{
     /// Unique identifier for the query to be answered
    pub pre_checkout_query_id: String,
     /// Specify True if everything is alright (goods are available, etc.) and the bot is ready to proceed with the order. Use False if there are any problems.
    pub ok: bool,
     /// Required if ok is False. Error message in human readable form that explains the reason for failure to proceed with the checkout (e.g. "Sorry, somebody just bought the last of our amazing black T-shirts while you were busy filling out your payment details. Please choose a different color or garment!"). Telegram will display this message to the user.
    pub error_message: Option<String>
}

impl AnswerPreCheckoutQuery {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "AnswerPreCheckoutQuery")
}}