

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct KeyboardButtonPollType{
    pub r#type: Option<String>
}