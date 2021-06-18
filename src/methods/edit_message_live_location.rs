use crate::types::{InlineKeyboardMarkup, Location, Message};
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "EditMessageLiveLocation does nothing until you `send` it"]
#[derive(Serialize)]
pub struct EditMessageLiveLocation<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// Required if inline_message_id is not specified. Identifier of the message to edit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// Latitude of new location
    pub latitude: f64,
    /// Longitude of new location
    pub longitude: f64,
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    /// Direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<i32>,
    /// Maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i32>,
    /// A JSON-serialized object for a new inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl<'a> EditMessageLiveLocation<'a> {
    pub fn new(bot: &'a Bot, latitude: f64, longitude: f64) -> Self {
        Self {
            latitude: latitude,
            longitude: longitude,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            horizontal_accuracy: None,
            heading: None,
            proximity_alert_radius: None,
            reply_markup: None,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<Message> {
        let string = serde_json::to_string(&self)?;
        let resp = self
            .bot
            .send("editMessageLiveLocation", Some(string))
            .await?;
        Ok(serde_json::from_str::<Message>(&resp.text().await?)?)
    }
    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = Some(chat_id);
        self
    }
    pub fn message_id(mut self, message_id: i32) -> Self {
        self.message_id = Some(message_id);
        self
    }
    pub fn inline_message_id(mut self, inline_message_id: String) -> Self {
        self.inline_message_id = Some(inline_message_id);
        self
    }
    pub fn latitude(mut self, latitude: f64) -> Self {
        self.latitude = latitude;
        self
    }
    pub fn longitude(mut self, longitude: f64) -> Self {
        self.longitude = longitude;
        self
    }
    pub fn horizontal_accuracy(mut self, horizontal_accuracy: f64) -> Self {
        self.horizontal_accuracy = Some(horizontal_accuracy);
        self
    }
    pub fn heading(mut self, heading: i32) -> Self {
        self.heading = Some(heading);
        self
    }
    pub fn proximity_alert_radius(mut self, proximity_alert_radius: i32) -> Self {
        self.proximity_alert_radius = Some(proximity_alert_radius);
        self
    }
    pub fn reply_markup(mut self, reply_markup: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }
}
