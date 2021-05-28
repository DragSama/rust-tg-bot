use crate::types::{File, InlineKeyboardMarkup, InputFile, Sticker, User};
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct SendSticker {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i32,
    /// Sticker to send. Pass a file_id as String to send a file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a .WEBP file from the Internet, or upload a new one using multipart/form-data. More info on Sending Files
    pub sticker: InputFile,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i32>,
    /// Pass True, if the message should be sent even if the specified replied-to message is not found
    pub allow_sending_without_reply: Option<bool>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl SendSticker {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "SendSticker")
    }
}
