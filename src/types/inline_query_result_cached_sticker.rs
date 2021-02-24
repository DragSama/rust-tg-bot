#[derive(Debug, Serialize)]
pub struct InlineQueryResultCachedSticker{
    pub type: String,
    pub id: String,
    pub sticker_file_id: String,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>
}