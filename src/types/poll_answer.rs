use crate::types::{user::User}

#[derive(Debug, Deserialize)]
pub struct PollAnswer{
    pub poll_id: String,
    pub user: User,
    pub option_ids: Vec<i32>
}