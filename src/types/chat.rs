use crate::types::{ChatLocation, ChatPermissions, ChatPhoto, Location, Message, User};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Chat {
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// Type of chat, can be either "private", "group", "supergroup" or "channel"
    pub r#type: String,
    /// Optional. Title, for supergroups, channels and group chats
    pub title: Option<String>,
    /// Optional. Username, for private chats, supergroups and channels if available
    pub username: Option<String>,
    /// Optional. First name of the other party in a private chat
    pub first_name: Option<String>,
    /// Optional. Last name of the other party in a private chat
    pub last_name: Option<String>,
    /// Optional. Chat photo. Returned only in getChat.
    pub photo: Option<ChatPhoto>,
    /// Optional. Bio of the other party in a private chat. Returned only in getChat.
    pub bio: Option<String>,
    /// Optional. Description, for groups, supergroups and channel chats. Returned only in getChat.
    pub description: Option<String>,
    /// Optional. Primary invite link, for groups, supergroups and channel chats. Returned only in getChat.
    pub invite_link: Option<String>,
    /// Optional. The most recent pinned message (by sending date). Returned only in getChat.
    pub pinned_message: Option<Box<Message>>,
    /// Optional. Default chat member permissions, for groups and supergroups. Returned only in getChat.
    pub permissions: Option<ChatPermissions>,
    /// Optional. For supergroups, the minimum allowed delay between consecutive messages sent by each unpriviledged user. Returned only in getChat.
    pub slow_mode_delay: Option<i32>,
    /// Optional. The time after which all messages sent to the chat will be automatically deleted; in seconds. Returned only in getChat.
    pub message_auto_delete_time: Option<i32>,
    /// Optional. For supergroups, name of group sticker set. Returned only in getChat.
    pub sticker_set_name: Option<String>,
    /// Optional. True, if the bot can change the group sticker set. Returned only in getChat.
    pub can_set_sticker_set: Option<bool>,
    /// Optional. Unique identifier for the linked chat, i.e. the discussion group identifier for a channel and vice versa; for supergroups and channel chats. This identifier may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier. Returned only in getChat.
    pub linked_chat_id: Option<i64>,
    /// Optional. For supergroups, the location to which the supergroup is connected. Returned only in getChat.
    pub location: Option<ChatLocation>,
}
