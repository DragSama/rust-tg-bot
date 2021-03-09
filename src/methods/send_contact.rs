use crate::types::{Contact, InlineKeyboardMarkup, User};
use serde::Serialize;
use serde_json;


#[derive(Debug, Serialize)]
pub struct SendContact{
     /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i32,
     /// Contact's phone number
    pub phone_number: String,
     /// Contact's first name
    pub first_name: String,
     /// Contact's last name
    pub last_name: Option<String>,
     /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    pub vcard: Option<String>,
     /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
     /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i32>,
     /// Pass True, if the message should be sent even if the specified replied-to message is not found
    pub allow_sending_without_reply: Option<bool>,
     /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove keyboard or to force a reply from the user.
    pub reply_markup: Option<InlineKeyboardMarkup>
}

impl SendContact {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "SendContact")
}}