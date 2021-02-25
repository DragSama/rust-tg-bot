use serde_json;
#[derive(Debug, Serialize)]
pub struct setChatAdministratorCustomTitle{
    pub chat_id: i64,
    pub user_id: i64,
    pub custom_title: String
}