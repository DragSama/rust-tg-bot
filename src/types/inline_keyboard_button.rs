use crate::types::{game::Game, login_url::LoginUrl}

#[derive(Debug, Deserialize)]
pub struct InlineKeyboardButton{
    pub text: String,
    pub url: Option<String>,
    pub login_url: Option<LoginUrl>,
    pub callback_data: Option<String>,
    pub switch_inline_query: Option<String>,
    pub switch_inline_query_current_chat: Option<String>,
    pub callback_game: Option<CallbackGame>,
    pub pay: Option<bool>
}