use crate::types::{Location};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ChatLocation{
    pub location: Location,
    pub address: String
}