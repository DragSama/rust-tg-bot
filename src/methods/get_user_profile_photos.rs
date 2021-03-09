use crate::types::{User, UserProfilePhotos};
use serde::Serialize;
use serde_json;


#[derive(Debug, Serialize)]
pub struct GetUserProfilePhotos{
     /// Unique identifier of the target user
    pub user_id: i32,
     /// Sequential number of the first photo to be returned. By default, all photos are returned.
    pub offset: Option<i32>,
     /// Limits the number of photos to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    pub limit: Option<i32>
}

impl GetUserProfilePhotos {
    pub fn get_data(&self) -> (String, &str) {
        (serde_json::to_string(&self).unwrap(), "GetUserProfilePhotos")
}}