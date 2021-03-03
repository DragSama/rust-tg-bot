use serde_json;

#[derive(Debug, Serialize)]
pub struct sendInvoice{
    pub chat_id: i32,
    pub title: String,
    pub description: String,
    pub payload: String,
    pub provider_token: String,
    pub start_parameter: String,
    pub currency: String,
    pub prices: Vec<LabeledPrice>,
    pub provider_data: Option<String>,
    pub photo_url: Option<String>,
    pub photo_size: Option<i32>,
    pub photo_width: Option<i32>,
    pub photo_height: Option<i32>,
    pub need_name: Option<bool>,
    pub need_phone_number: Option<bool>,
    pub need_email: Option<bool>,
    pub need_shipping_address: Option<bool>,
    pub send_phone_number_to_provider: Option<bool>,
    pub send_email_to_provider: Option<bool>,
    pub is_flexible: Option<bool>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i32>,
    pub allow_sending_without_reply: Option<bool>,
    pub reply_markup: Option<InlineKeyboardMarkup>
}