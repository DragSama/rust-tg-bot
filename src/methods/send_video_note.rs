use crate::types::{File, InlineKeyboardMarkup, InputFile, User, Video, VideoNote};
use serde::Serialize;
use serde_json;


#[derive(Debug, Serialize)]
pub struct SendVideoNote{
     /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i32,
     /// Video note to send. Pass a file_id as String to send a video note that exists on the Telegram servers (recommended) or upload a new video using multipart/form-data. More info on Sending Files . Sending video notes by a URL is currently unsupported
    pub video_note: InputFile,
     /// Duration of sent video in seconds
    pub duration: Option<i32>,
     /// Video width and height, i.e. diameter of the video message
    pub length: Option<i32>,
     /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass "attach://<file_attach_name>" if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More info on Sending Files 
    pub thumb: Option<InputFile>,
     /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
     /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i32>,
     /// Pass True, if the message should be sent even if the specified replied-to message is not found
    pub allow_sending_without_reply: Option<bool>,
     /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<InlineKeyboardMarkup>
}

impl SendVideoNote {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "SendVideoNote")
}}