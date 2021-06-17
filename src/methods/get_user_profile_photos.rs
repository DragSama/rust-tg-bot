use crate::types::{User, UserProfilePhotos};
use crate::{bot::Bot, error::Result};
use serde::Serialize;
use serde_json;

#[must_use = "GetUserProfilePhotos does nothing until you `send` it"]
#[derive(Serialize)]
pub struct GetUserProfilePhotos<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier of the target user
    pub user_id: i32,
    /// Sequential number of the first photo to be returned. By default, all photos are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Limits the number of photos to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

impl<'a> GetUserProfilePhotos<'a> {
    pub fn new(bot: &'a Bot, user_id: i32) -> Self {
        Self {
            user_id: user_id,
            offset: None,
            limit: None,
            bot: bot,
        }
    }
    pub async fn send(self) -> Result<UserProfilePhotos> {
        let string = serde_json::to_string(&self)?;
        let resp = self.bot.send("getUserProfilePhotos", Some(string)).await?;
        Ok(serde_json::from_str::<UserProfilePhotos>(
            &resp.text().await?,
        )?)
    }
    pub fn user_id(mut self, user_id: i32) -> Self {
        self.user_id = user_id;
        self
    }
    pub fn offset(mut self, offset: i32) -> Self {
        self.offset = Some(offset);
        self
    }
    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }
}
