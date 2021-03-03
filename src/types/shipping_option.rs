use crate::types::{LabeledPrice};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ShippingOption{
    pub id: String,
    pub title: String,
    pub prices: Vec<LabeledPrice>
}