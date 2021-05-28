use crate::types::{File, InlineKeyboardMarkup, InputFile, Message, MessageEntity, User, Video};
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct SendVideo {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i32,
    /// Video to send. Pass a file_id as String to send a video that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a video from the Internet, or upload a new video using multipart/form-data. More info on Sending Files
    pub video: InputFile,
    /// Duration of sent video in seconds
    pub duration: Option<i32>,
    /// Video width
    pub width: Option<i32>,
    /// Video height
    pub height: Option<i32>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass "attach://<file_attach_name>" if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More info on Sending Files
    pub thumb: Option<InputFile>,
    /// Video caption (may also be used when resending videos by file_id), 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the video caption. See formatting options for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Pass True, if the uploaded video is suitable for streaming
    pub supports_streaming: Option<bool>,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i32>,
    /// Pass True, if the message should be sent even if the specified replied-to message is not found
    pub allow_sending_without_reply: Option<bool>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl SendVideo {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "SendVideo")
    }
}
