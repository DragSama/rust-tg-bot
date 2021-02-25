use serde_json;
#[derive(Debug, Serialize)]
pub struct answerInlineQuery{
    pub inline_query_id: String,
    pub results: Vec<InlineQueryResult>,
    pub cache_time: Option<i64>,
    pub is_personal: Option<bool>,
    pub next_offset: Option<String>,
    pub switch_pm_text: Option<String>,
    pub switch_pm_parameter: Option<String>
}