use crate::types::{keyboard_button_poll_type::KeyboardButtonPollType, poll::Poll}

#[derive(Debug, Deserialize)]
pub struct KeyboardButton{
    pub text: String,
    pub request_contact: Option<bool>,
    pub request_location: Option<bool>,
    pub request_poll: Option<KeyboardButtonPollType>
}