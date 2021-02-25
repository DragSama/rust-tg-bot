use crate::types::{callback_query::CallbackQuery, chosen_inline_result::ChosenInlineResult, inline_query::InlineQuery, message::Message, poll::Poll, poll_answer::PollAnswer, pre_checkout_query::PreCheckoutQuery, shipping_query::ShippingQuery}

#[derive(Debug, Serialize)]
pub struct Update{
    pub update_id: i64,
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