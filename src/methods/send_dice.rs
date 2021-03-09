use crate::types::{Dice, InlineKeyboardMarkup, User};
use serde::Serialize;
use serde_json;


#[derive(Debug, Serialize)]
pub struct SendDice{
     /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i32,
     /// Emoji on which the dice throw animation is based. Currently, must be one of "", "", "", "", or "". Dice can have values 1-6 for "" and "", values 1-5 for "" and "", and values 1-64 for "". Defaults to ""
    pub emoji: Option<String>,
     /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
     /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i32>,
     /// Pass True, if the message should be sent even if the specified replied-to message is not found
    pub allow_sending_without_reply: Option<bool>,
     /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<InlineKeyboardMarkup>
}

impl SendDice {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "SendDice")
}}