use crate::types::{labeled_price::LabeledPrice}

#[derive(Debug, Deserialize)]
pub struct ShippingOption{
    pub id: String,
    pub title: String,
    pub prices: Vec<LabeledPrice>
}