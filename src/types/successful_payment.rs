use crate::types::{OrderInfo};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SuccessfulPayment{
    pub currency: String,
    pub total_amount: i32,
    pub invoice_payload: String,
    pub shipping_option_id: Option<String>,
    pub order_info: Option<OrderInfo>,
    pub telegram_payment_charge_id: String,
    pub provider_payment_charge_id: String
}