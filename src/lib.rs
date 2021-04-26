#![allow(unused_imports)]
#![allow(dead_code)]

extern crate reqwest;
extern crate serde_json;

pub mod types;
pub mod methods;

use crate::types::{Update, Updates};

pub struct Bot {
    pub token: String
}

impl Bot {
    pub fn new(bot_token: &str) -> Self {
        Self {
            token: bot_token.into()
        }
    }
}

pub struct Updater {
    pub bot: Bot,
    pub base_endpoint: String,
    pub reqwest_client: reqwest::Client,
}

impl Updater {
    pub fn new(bot_token: &str) -> Self {
        Self {
            bot: Bot::new(bot_token.into()),
            base_endpoint: format!("https://api.telegram.org/bot{}/", bot_token).into(),
            reqwest_client: reqwest::Client::new(),
        }
    }
    async fn post(self, path: &str, data: String) -> String{
        let url = format!("{}{}", self.base_endpoint, path);
        self.reqwest_client.post(&url)
                .header("Content-Type", "application/json")
                .header("Accept", "application/json")
                .body(data)
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap()
    }

    pub async fn handle_update(&self, update: &Update){
       println!("Got update: {:#?}", update)
    }
    pub async fn start_polling(self){
        let mut offset: i32 = 0;
        loop {
            let url = format!("{}{}?offset={}", self.base_endpoint, "getUpdates", offset);
            let result = self.reqwest_client.get(&url).send().await.unwrap().text().await.unwrap();
            let mut resp = serde_json::from_str::<Updates>(&result).unwrap();
            for update in resp.result.iter() {
                self.handle_update(update).await;
                offset = update.update_id
            }
        }
    }
}
