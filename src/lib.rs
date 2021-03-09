#![allow(unused_imports)]
#![allow(dead_code)]

extern crate reqwest;
extern crate serde_json;

pub mod types;
pub mod methods;

use crate::types::{Update, Updates};

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
    pub async fn send_message(self, chat_id: i64, text: &str, parse_mode: Option<String>, disable_web_page_preview: Option<bool>) -> String{
        let method = methods::SendMessage{
            chat_id: chat_id,
            text: text.into(),
            parse_mode: parse_mode,
            disable_web_page_preview: disable_web_page_preview,
            disable_notification: Some(false),
            reply_to_message_id: None,
            allow_sending_without_reply: Some(true),
            reply_markup: None,
            entities: None
        };
        let data: (String, &str) = method.get_data();
        self.post(&data.1, data.0).await
    }
    pub async fn handle_update(&self, update: &Update){
       println!("Got update: {:#?}", update)
    }
    pub async fn start_polling(self){
        let mut offset: i32 = -1;
        loop {
            let url = format!("{}{}?offset={}", self.base_endpoint, "getUpdates", offset);
            let result = self.reqwest_client.get(&url).send().await.unwrap().text().await.unwrap();
            let mut resp = serde_json::from_str::<Updates>(&result).unwrap();
            resp.result.reverse();
            for update in resp.result.iter() {
                self.handle_update(update).await;
                if offset == -1 {
                    offset = update.update_id;
                } else {
                    offset = offset + 1;
                }
            }
        }
    }
}
// fn main() {
//     println!("Hello, world!");
// }
