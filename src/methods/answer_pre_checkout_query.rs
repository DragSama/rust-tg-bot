use crate::types::{PreCheckoutQuery};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct answerPreCheckoutQuery{
    pub pre_checkout_query_id: String,
    pub ok: bool,
    pub error_message: Option<String>
}