use crate::types::Chat;

use reqwest::Error as ReqwestError;
use serde_json::Error as JsonError;

use std::fmt;

#[derive(Debug)]
pub enum Error {
    TelegramError(String), //  default error for from, specific error to be preferred
    Unauthorised(String),
    InvalidToken,
    BadRequest(String),
    TimedOut,
    ChatMigrated(i64),
    RetryAfter(i64),
    Conflict(String),
    RequestError(ReqwestError),
    SerdeJsonError(JsonError),
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::TelegramError(msg) => write!(f, "{}", msg),
            Self::RequestError(rq) => write!(f, "Failure making request: {:?}", rq),
            Self::ChatMigrated(new_chat_id) => write!(
                f,
                "Group Migrated To SuperGroup. New Chat ID: {}",
                new_chat_id
            ),
            Self::Unauthorised(msg) => write!(f, "{}", msg),
            Self::InvalidToken => write!(f, "Invalid Token"),
            Self::BadRequest(msg) => write!(f, "{}", msg),
            Self::RetryAfter(retry_after) => write!(
                f,
                "Flood Control Exceeded. Retry in {} seconds.",
                retry_after
            ),
            Self::Conflict(msg) => write!(f, "{}", msg),
            Self::TimedOut => write!(f, "Timed Out"),
            Self::SerdeJsonError(obj) => write!(f, "Failed to parse or construct JSON: {:?}", obj),
        }
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Self::TelegramError(err)
    }
}

impl From<ReqwestError> for Error {
    fn from(error: ReqwestError) -> Self {
        Self::RequestError(error)
    }
}

impl From<JsonError> for Error {
    fn from(error: JsonError) -> Self {
        Self::SerdeJsonError(error)
    }
}
