use serde_json;
#[derive(Debug, Serialize)]
pub struct setChatPermissions{
    pub chat_id: i64,
    pub permissions: ChatPermissions
}