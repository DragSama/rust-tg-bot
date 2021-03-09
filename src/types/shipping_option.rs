use crate::types::{LabeledPrice};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ShippingOption{
    /// Shipping option identifier
    pub id: String,
    /// Option title
    pub title: String,
    /// List of price portions
    pub prices: Vec<LabeledPrice>
}