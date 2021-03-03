use crate::types::{ShippingAddress};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OrderInfo{
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub shipping_address: Option<ShippingAddress>
}