use crate::types::{InlineKeyboardMarkup, Message, MessageEntity};
use serde::Serialize;
use serde_json;


#[derive(Debug, Serialize)]
pub struct EditMessageCaption{
     /// Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: Option<i32>,
     /// Required if inline_message_id is not specified. Identifier of the message to edit
    pub message_id: Option<i32>,
     /// Required if chat_id and message_id are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
     /// New caption of the message, 0-1024 characters after entities parsing
    pub caption: Option<String>,
     /// Mode for parsing entities in the message caption. See formatting options for more details.
    pub parse_mode: Option<String>,
     /// List of special entities that appear in the caption, which can be specified instead of parse_mode
    pub caption_entities: Option<Vec<MessageEntity>>,
     /// A JSON-serialized object for an inline keyboard.
    pub reply_markup: Option<InlineKeyboardMarkup>
}

impl EditMessageCaption {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "EditMessageCaption")
}}