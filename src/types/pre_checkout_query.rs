#[derive(Debug, Serialize)]
pub struct PreCheckoutQuery{
    pub id: String,
    pub from: User,
    pub currency: String,
    pub total_amount: i64,
    pub invoice_payload: String,
    pub shipping_option_id: Option<String>,
    pub order_info: Option<OrderInfo>
}