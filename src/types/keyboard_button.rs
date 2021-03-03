use crate::types::{KeyboardButtonPollType, Poll};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct KeyboardButton{
    pub text: String,
    pub request_contact: Option<bool>,
    pub request_location: Option<bool>,
    pub request_poll: Option<KeyboardButtonPollType>
}