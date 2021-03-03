use serde_json;

#[derive(Debug, Serialize)]
pub struct answerPreCheckoutQuery{
    pub pre_checkout_query_id: String,
    pub ok: bool,
    pub error_message: Option<String>
}