use serde_json;
#[derive(Debug, Serialize)]
pub struct sendSticker{
    pub chat_id: i64,
    pub sticker: InputFile,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub allow_sending_without_reply: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>
}