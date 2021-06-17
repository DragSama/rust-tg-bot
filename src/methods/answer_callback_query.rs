use crate::types::{CallbackQuery, Game};
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "AnswerCallbackQuery does nothing until you `send` it"]
#[derive(Serialize)]
pub struct AnswerCallbackQuery<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the query to be answered
    pub callback_query_id: String,
    /// Text of the notification. If not specified, nothing will be shown to the user, 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// If true, an alert will be shown by the client instead of a notification at the top of the chat screen. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_alert: Option<bool>,
    /// URL that will be opened by the user's client. If you have created a Game and accepted the conditions via @Botfather, specify the URL that opens your game  note that this will only work if the query comes from a callback_game button.Otherwise, you may use links like t.me/your_bot?start=XXXX that open your bot with a parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The maximum amount of time in seconds that the result of the callback query may be cached client-side. Telegram apps will support caching starting in version 3.14. Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i32>,
}

impl<'a> AnswerCallbackQuery<'a> {
    pub fn new(bot: &'a Bot, callback_query_id: String) -> Self {
        Self {
            callback_query_id: callback_query_id,
            text: None,
            show_alert: None,
            url: None,
            cache_time: None,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<bool> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("answerCallbackQuery", Some(string)).await?;
        Ok(serde_json::from_str::<bool>(&resp.text().await?)?)
    }
    pub fn callback_query_id(mut self, callback_query_id: String) -> Self {
        self.callback_query_id = callback_query_id;
        self
    }
    pub fn text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }
    pub fn show_alert(mut self, show_alert: bool) -> Self {
        self.show_alert = Some(show_alert);
        self
    }
    pub fn url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }
    pub fn cache_time(mut self, cache_time: i32) -> Self {
        self.cache_time = Some(cache_time);
        self
    }
}
