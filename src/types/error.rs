use std::fmt;

#[derive(Debug)]
pub enum Error {
    TelegramError(String), //  default error for from, specific error to be preferred
    Unauthorised(String),
    InvalidToken,
    NetworkError(String),
    BadRequest(String),
    TimedOut,
    ChatMigrated(i64),
    RetryAfter(i64),
    Conflict(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::TelegramError(msg) => write!(f, "{}", msg),
            Self::Unauthorised(msg) => write!(f, "{}", msg),
            Self::InvalidToken => write!(f, "Invalid Token"),
            Self::NetworkError(msg) => write!(f, "{}", msg),
            Self::BadRequest(msg) => write!(f, "{}", msg),
            Self::TimedOut => write!(f, "Timed Out"),
            Self::ChatMigrated(new_chat_id) => write!(f, "Group Migrated To SuperGroup. New Chat ID: {}", new_chat_id),
            Self::RetryAfter(retry_after) => write!(f, "Flood Control Exceeded. Retry in {} seconds.", retry_after),
            Self::Conflict(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Self::TelegramError(err)
    }
}
