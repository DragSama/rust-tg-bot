use serde_json;
#[derive(Debug, Serialize)]
pub struct setPassportDataErrors{
    pub user_id: i64,
    pub errors: Vec<PassportElementError>
}