#[derive(Debug, Deserialize)]
pub struct LabeledPrice{
    pub label: String,
    pub amount: i32
}