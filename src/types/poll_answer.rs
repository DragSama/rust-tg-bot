#[derive(Debug, Serialize)]
pub struct PollAnswer{
    pub poll_id: String,
    pub user: User,
    pub option_ids: Vec<i64>
}