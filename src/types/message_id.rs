use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MessageId{
    pub message_id: i32
}