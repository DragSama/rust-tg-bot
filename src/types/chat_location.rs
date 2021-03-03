use serde::Deserialize;
use crate::types::Location;

#[derive(Debug, Deserialize)]
pub struct ChatLocation{
    pub location: Location,
    pub address: String
}
