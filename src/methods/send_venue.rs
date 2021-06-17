use crate::types::{InlineKeyboardMarkup, Message, User, Venue};
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "SendVenue does nothing until you `send` it"]
#[derive(Serialize)]
pub struct SendVenue<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i32,
    /// Latitude of the venue
    pub latitude: f64,
    /// Longitude of the venue
    pub longitude: f64,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue, if known. (For example, "arts_entertainment/default", "arts_entertainment/aquarium" or "food/icecream".)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    /// Google Places identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,
    /// Google Places type of the venue. (See supported types.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i32>,
    /// Pass True, if the message should be sent even if the specified replied-to message is not found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl<'a> SendVenue<'a> {
    pub fn new(
        bot: &'a Bot,
        chat_id: i32,
        latitude: f64,
        longitude: f64,
        title: String,
        address: String,
    ) -> Self {
        Self {
            chat_id: chat_id,
            latitude: latitude,
            longitude: longitude,
            title: title,
            address: address,
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<Message> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("sendVenue", Some(string)).await?;
        Ok(serde_json::from_str::<Message>(&resp.text().await?)?)
    }
    pub fn chat_id(mut self, chat_id: i32) -> Self {
        self.chat_id = chat_id;
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
    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }
    pub fn address(mut self, address: String) -> Self {
        self.address = address;
        self
    }
    pub fn foursquare_id(mut self, foursquare_id: String) -> Self {
        self.foursquare_id = Some(foursquare_id);
        self
    }
    pub fn foursquare_type(mut self, foursquare_type: String) -> Self {
        self.foursquare_type = Some(foursquare_type);
        self
    }
    pub fn google_place_id(mut self, google_place_id: String) -> Self {
        self.google_place_id = Some(google_place_id);
        self
    }
    pub fn google_place_type(mut self, google_place_type: String) -> Self {
        self.google_place_type = Some(google_place_type);
        self
    }
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }
    pub fn reply_to_message_id(mut self, reply_to_message_id: i32) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }
    pub fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }
    pub fn reply_markup(mut self, reply_markup: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }
}
