#[derive(Debug, Deserialize)]
pub struct PollOption{
    pub text: String,
    pub voter_count: i32
}