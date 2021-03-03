use crate::types::{CallbackQuery, ChosenInlineResult, InlineQuery, Message, Poll, PollAnswer, PreCheckoutQuery, ShippingQuery};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Update{
    pub update_id: i32,
    pub message: Option<Message>,
    pub edited_message: Option<Message>,
    pub channel_post: Option<Message>,
    pub edited_channel_post: Option<Message>,
    pub inline_query: Option<InlineQuery>,
    pub chosen_inline_result: Option<ChosenInlineResult>,
    pub callback_query: Option<CallbackQuery>,
    pub shipping_query: Option<ShippingQuery>,
    pub pre_checkout_query: Option<PreCheckoutQuery>,
    pub poll: Option<Poll>,
    pub poll_answer: Option<PollAnswer>
}