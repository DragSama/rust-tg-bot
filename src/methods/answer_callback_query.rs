use serde_json;
#[derive(Debug, Serialize)]
pub struct answerCallbackQuery{
    pub callback_query_id: String,
    pub text: Option<String>,
    pub show_alert: Option<bool>,
    pub url: Option<String>,
    pub cache_time: Option<i64>
}