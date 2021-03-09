use crate::types::{PassportData, PassportElementError, User};
use serde::Serialize;
use serde_json;


#[derive(Debug, Serialize)]
pub struct SetPassportDataErrors{
     /// User identifier
    pub user_id: i32,
     /// A JSON-serialized array describing the errors
    pub errors: Vec<PassportElementError>
}

impl SetPassportDataErrors {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "SetPassportDataErrors")
}}