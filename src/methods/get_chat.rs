use crate::types::Chat;
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct GetChat {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: i32,
}

impl GetChat {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "GetChat")
    }
}
