use serde_json;

#[derive(Debug, Serialize)]
pub struct setChatPermissions{
    pub chat_id: i32,
    pub permissions: ChatPermissions
}