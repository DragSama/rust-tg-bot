use serde_json;

#[derive(Debug, Serialize)]
pub struct setChatPhoto{
    pub chat_id: i32,
    pub photo: InputFile
}