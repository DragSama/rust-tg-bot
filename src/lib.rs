#![allow(unused_imports)]

extern crate reqwest;
extern crate serde_json;

pub mod types;

use crate::types::UpdateResp;

pub struct Updater {
    pub bot_token: String,
    pub base_endpoint: String,
    pub reqwest_client: reqwest::Client,
}

impl Updater {
    pub fn new(bot_token: &str) -> Self {
        Self {
            bot_token: bot_token.into(),
            base_endpoint: format!("https://api.telegram.org/bot{}/", bot_token).into(),
            reqwest_client: reqwest::Client::new(),
        }
    }
    pub async fn handle_update(&self, update: UpdateResp){
       println!("Got update: {:#?}", update)
    }
    pub async fn start_polling(self){
       loop {
           let url = format!("{}{}", self.base_endpoint, "getUpdates");
           let result = self.reqwest_client.get(&url).send().await.unwrap().text().await.unwrap();
           self.handle_update(serde_json::from_str::<UpdateResp>(&result).unwrap()).await;
       }
    }
}
// fn main() {
//     println!("Hello, world!");
// }
