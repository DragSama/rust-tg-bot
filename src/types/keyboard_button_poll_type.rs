use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct KeyboardButtonPollType{
    pub type: Option<String>
}