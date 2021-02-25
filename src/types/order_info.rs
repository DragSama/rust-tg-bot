use crate::types::{shipping_address::ShippingAddress}

#[derive(Debug, Deserialize)]
pub struct OrderInfo{
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub shipping_address: Option<ShippingAddress>
}