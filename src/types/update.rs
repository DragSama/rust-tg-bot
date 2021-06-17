use crate::types::{
    CallbackQuery, Chat, ChatMember, ChatMemberUpdated, ChosenInlineResult, InlineQuery, Message,
    Poll, PollAnswer, PreCheckoutQuery, ShippingQuery,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Update {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This ID becomes especially handy if you're using Webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// Optional. New incoming message of any kind  text, photo, sticker, etc.
    pub message: Option<Box<Message>>,
    /// Optional. New version of a message that is known to the bot and was edited
    pub edited_message: Option<Box<Message>>,
    /// Optional. New incoming channel post of any kind  text, photo, sticker, etc.
    pub channel_post: Option<Box<Message>>,
    /// Optional. New version of a channel post that is known to the bot and was edited
    pub edited_channel_post: Option<Box<Message>>,
    /// Optional. New incoming inline query
    pub inline_query: Option<Box<Message>>,
    /// Optional. The result of an inline query that was chosen by a user and sent to their chat partner. Please see our documentation on the feedback collecting for details on how to enable these updates for your bot.
    pub chosen_inline_result: Option<Box<Message>>,
    /// Optional. New incoming callback query
    pub callback_query: Option<Box<Message>>,
    /// Optional. New incoming shipping query. Only for invoices with flexible price
    pub shipping_query: Option<Box<Message>>,
    /// Optional. New incoming pre-checkout query. Contains full information about checkout
    pub pre_checkout_query: Option<Box<Message>>,
    /// Optional. New poll state. Bots receive only updates about stopped polls and polls, which are sent by the bot
    pub poll: Option<Poll>,
    /// Optional. A user changed their answer in a non-anonymous poll. Bots receive new votes only in polls that were sent by the bot itself.
    pub poll_answer: Option<PollAnswer>,
}

#[derive(Debug, Deserialize)]
pub struct Updates {
    pub ok: bool,
    pub result: Vec<Update>,
}
