use crate::types::{KeyboardButtonPollType, Poll};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KeyboardButton {
    /// Text of the button. If none of the optional fields are used, it will be sent as a message when the button is pressed
    pub text: String,
    /// Optional. If True, the user's phone number will be sent as a contact when the button is pressed. Available in private chats only
    pub request_contact: Option<bool>,
    /// Optional. If True, the user's current location will be sent when the button is pressed. Available in private chats only
    pub request_location: Option<bool>,
    /// Optional. If specified, the user will be asked to create a poll and send it to the bot when the button is pressed. Available in private chats only
    pub request_poll: Option<KeyboardButtonPollType>,
}
