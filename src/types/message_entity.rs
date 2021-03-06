use crate::types::{Message, User};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MessageEntity {
    /// Type of the entity. Can be "mention" (@username), "hashtag" (#hashtag), "cashtag" ($USD), "bot_command" (/start@jobs_bot), "url" (https://telegram.org), "email" (do-not-reply@telegram.org), "phone_number" (+1-212-555-0123), "bold" (bold text), "italic" (italic text), "underline" (underlined text), "strikethrough" (strikethrough text), "code" (monowidth string), "pre" (monowidth block), "text_link" (for clickable text URLs), "text_mention" (for users without usernames)
    pub r#type: String,
    /// Offset in UTF-16 code units to the start of the entity
    pub offset: i32,
    /// Length of the entity in UTF-16 code units
    pub length: i32,
    /// Optional. For "text_link" only, url that will be opened after user taps on the text
    pub url: Option<String>,
    /// Optional. For "text_mention" only, the mentioned user
    pub user: Option<User>,
    /// Optional. For "pre" only, the programming language of the entity text
    pub language: Option<String>,
}
