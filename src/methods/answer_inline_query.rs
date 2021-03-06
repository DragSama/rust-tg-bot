use crate::types::{InlineQuery, InlineQueryResult};
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "AnswerInlineQuery does nothing until you `send` it"]
#[derive(Serialize)]
pub struct AnswerInlineQuery<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the answered query
    pub inline_query_id: String,
    /// A JSON-serialized array of results for the inline query
    pub results: Vec<InlineQueryResult>,
    /// The maximum amount of time in seconds that the result of the inline query may be cached on the server. Defaults to 300.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i32>,
    /// Pass True, if results may be cached on the server side only for the user that sent the query. By default, results may be returned to any user who sends the same query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_personal: Option<bool>,
    /// Pass the offset that a client should send in the next query with the same text to receive more results. Pass an empty string if there are no more results or if you don't support pagination. Offset length can't exceed 64 bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<String>,
    /// If passed, clients will display a button with specified text that switches the user to a private chat with the bot and sends the bot a start message with the parameter switch_pm_parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_pm_text: Option<String>,
    /// Deep-linking parameter for the /start message sent to the bot when user presses the switch button. 1-64 characters, only A-Z, a-z, 0-9, _ and - are allowed.Example: An inline bot that sends YouTube videos can ask the user to connect the bot to their YouTube account to adapt search results accordingly. To do this, it displays a 'Connect your YouTube account' button above the results, or even before showing any. The user presses the button, switches to a private chat with the bot and, in doing so, passes a start parameter that instructs the bot to return an oauth link. Once done, the bot can offer a switch_inline button so that the user can easily return to the chat where they wanted to use the bot's inline capabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_pm_parameter: Option<String>,
}

impl<'a> AnswerInlineQuery<'a> {
    pub fn new(bot: &'a Bot, inline_query_id: String, results: Vec<InlineQueryResult>) -> Self {
        Self {
            inline_query_id: inline_query_id,
            results: results,
            cache_time: None,
            is_personal: None,
            next_offset: None,
            switch_pm_text: None,
            switch_pm_parameter: None,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<bool> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("answerInlineQuery", Some(string)).await?;
        Ok(serde_json::from_str::<bool>(&resp.text().await?)?)
    }
    pub fn inline_query_id(mut self, inline_query_id: String) -> Self {
        self.inline_query_id = inline_query_id;
        self
    }
    pub fn results(mut self, results: Vec<InlineQueryResult>) -> Self {
        self.results = results;
        self
    }
    pub fn cache_time(mut self, cache_time: i32) -> Self {
        self.cache_time = Some(cache_time);
        self
    }
    pub fn is_personal(mut self, is_personal: bool) -> Self {
        self.is_personal = Some(is_personal);
        self
    }
    pub fn next_offset(mut self, next_offset: String) -> Self {
        self.next_offset = Some(next_offset);
        self
    }
    pub fn switch_pm_text(mut self, switch_pm_text: String) -> Self {
        self.switch_pm_text = Some(switch_pm_text);
        self
    }
    pub fn switch_pm_parameter(mut self, switch_pm_parameter: String) -> Self {
        self.switch_pm_parameter = Some(switch_pm_parameter);
        self
    }
}
