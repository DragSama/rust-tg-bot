use crate::types::{File, InputFile, InputMedia, Message, MessageEntity, Video};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InputMediaVideo {
    /// Type of the result, must be video
    pub r#type: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass "attach://<file_attach_name>" to upload a new one using multipart/form-data under <file_attach_name> name. More info on Sending Files
    pub media: String,
    /// Optional. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass "attach://<file_attach_name>" if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More info on Sending Files
    #[serde(skip)]
    pub thumb: Option<InputFile>,
    /// Optional. Caption of the video to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the video caption. See formatting options for more details.
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Video width
    pub width: Option<i32>,
    /// Optional. Video height
    pub height: Option<i32>,
    /// Optional. Video duration
    pub duration: Option<i32>,
    /// Optional. Pass True, if the uploaded video is suitable for streaming
    pub supports_streaming: Option<bool>,
}
