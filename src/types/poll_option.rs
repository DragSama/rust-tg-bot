#[derive(Debug, Serialize)]
pub struct PollOption{
    pub text: String,
    pub voter_count: i64
}