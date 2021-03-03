use crate::types::{ChatLocation, ChatPermissions, ChatPhoto, Location, Message};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Chat{
    pub id: i32,
    pub type: String,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub photo: Option<ChatPhoto>,
    pub bio: Option<String>,
    pub description: Option<String>,
    pub invite_link: Option<String>,
    pub pinned_message: Option<Message>,
    pub permissions: Option<ChatPermissions>,
    pub slow_mode_delay: Option<i32>,
    pub sticker_set_name: Option<String>,
    pub can_set_sticker_set: Option<bool>,
    pub linked_chat_id: Option<i32>,
    pub location: Option<ChatLocation>
}