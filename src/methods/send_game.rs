use serde_json;

#[derive(Debug, Serialize)]
pub struct sendGame{
    pub chat_id: i32,
    pub game_short_name: String,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i32>,
    pub allow_sending_without_reply: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>
}