use serde_json;

#[derive(Debug, Serialize)]
pub struct sendVideoNote{
    pub chat_id: i32,
    pub video_note: InputFile,
    pub duration: Option<i32>,
    pub length: Option<i32>,
    pub thumb: Option<InputFile>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i32>,
    pub allow_sending_without_reply: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>
}