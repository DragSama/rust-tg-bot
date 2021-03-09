use crate::types::{InlineKeyboardMarkup, Message, MessageEntity, Poll, User};
use serde::Serialize;
use serde_json;


#[derive(Debug, Serialize)]
pub struct SendPoll{
     /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i32,
     /// Poll question, 1-300 characters
    pub question: String,
     /// A JSON-serialized list of answer options, 2-10 strings 1-100 characters each
    pub options: Vec<String>,
     /// True, if the poll needs to be anonymous, defaults to True
    pub is_anonymous: Option<bool>,
     /// Poll type, "quiz" or "regular", defaults to "regular"
    pub r#type: Option<String>,
     /// True, if the poll allows multiple answers, ignored for polls in quiz mode, defaults to False
    pub allows_multiple_answers: Option<bool>,
     /// 0-based identifier of the correct answer option, required for polls in quiz mode
    pub correct_option_id: Option<i32>,
     /// Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters with at most 2 line feeds after entities parsing
    pub explanation: Option<String>,
     /// Mode for parsing entities in the explanation. See formatting options for more details.
    pub explanation_parse_mode: Option<String>,
     /// List of special entities that appear in the poll explanation, which can be specified instead of parse_mode
    pub explanation_entities: Option<Vec<MessageEntity>>,
     /// Amount of time in seconds the poll will be active after creation, 5-600. Can't be used together with close_date.
    pub open_period: Option<i32>,
     /// Point in time (Unix timestamp) when the poll will be automatically closed. Must be at least 5 and no more than 600 seconds in the future. Can't be used together with open_period.
    pub close_date: Option<i32>,
     /// Pass True, if the poll needs to be immediately closed. This can be useful for poll preview.
    pub is_closed: Option<bool>,
     /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
     /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i32>,
     /// Pass True, if the message should be sent even if the specified replied-to message is not found
    pub allow_sending_without_reply: Option<bool>,
     /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<InlineKeyboardMarkup>
}

impl SendPoll {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "SendPoll")
}}
