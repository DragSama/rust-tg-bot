use serde_json;

#[derive(Debug, Serialize)]
pub struct setPassportDataErrors{
    pub user_id: i32,
    pub errors: Vec<PassportElementError>
}