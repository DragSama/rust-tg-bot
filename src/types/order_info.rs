#[derive(Debug, Serialize)]
pub struct OrderInfo{
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub shipping_address: Option<ShippingAddress>
}