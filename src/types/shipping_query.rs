use crate::types::{shipping_address::ShippingAddress, user::User}

#[derive(Debug, Deserialize)]
pub struct ShippingQuery{
    pub id: String,
    pub from: User,
    pub invoice_payload: String,
    pub shipping_address: ShippingAddress
}