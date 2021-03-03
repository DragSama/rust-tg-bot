use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Dice{
    pub emoji: String,
    pub value: i32
}