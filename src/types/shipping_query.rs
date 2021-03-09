use crate::types::{ShippingAddress, User};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ShippingQuery{
    /// Unique query identifier
    pub id: String,
    /// User who sent the query
    pub from: User,
    /// Bot specified invoice payload
    pub invoice_payload: String,
    /// User specified shipping address
    pub shipping_address: ShippingAddress
}