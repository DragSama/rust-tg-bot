use crate::types::{User};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PollAnswer{
    pub poll_id: String,
    pub user: User,
    pub option_ids: Vec<i32>
}