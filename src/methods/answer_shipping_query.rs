use serde_json;

#[derive(Debug, Serialize)]
pub struct answerShippingQuery{
    pub shipping_query_id: String,
    pub ok: bool,
    pub shipping_options: Option<Vec<ShippingOption>>,
    pub error_message: Option<String>
}