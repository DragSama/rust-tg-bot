#[derive(Debug, Serialize)]
pub struct ForceReply{
    pub force_reply: bool,
    pub selective: Option<bool>
}