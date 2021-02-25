use serde_json;
#[derive(Debug, Serialize)]
pub struct getUserProfilePhotos{
    pub user_id: i64,
    pub offset: Option<i64>,
    pub limit: Option<i64>
}