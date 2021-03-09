use crate::types::{ShippingOption, ShippingQuery};
use serde::Serialize;
use serde_json;


#[derive(Debug, Serialize)]
pub struct AnswerShippingQuery{
     /// Unique identifier for the query to be answered
    pub shipping_query_id: String,
     /// Specify True if delivery to the specified address is possible and False if there are any problems (for example, if delivery to the specified address is not possible)
    pub ok: bool,
     /// Required if ok is True. A JSON-serialized array of available shipping options.
    pub shipping_options: Option<Vec<ShippingOption>>,
     /// Required if ok is False. Error message in human readable form that explains why it is impossible to complete the order (e.g. "Sorry, delivery to your desired address is unavailable'). Telegram will display this message to the user.
    pub error_message: Option<String>
}

impl AnswerShippingQuery {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "AnswerShippingQuery")
}}