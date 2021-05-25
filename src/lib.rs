#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate reqwest;
extern crate serde_json;

pub mod types;
pub mod methods;

use crate::types::{Update, Updates};
use std::{cmp::max};

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

pub struct CommandHandler {
    pub command: String,
    pub func: fn(&Update)
}

impl CommandHandler {
    pub fn check_update(&self, update: &Update){
        true;
    }
}

pub struct Dispatcher {
    pub handlers: Vec<CommandHandler>
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new()
        }
    }
    pub fn add_handler(&mut self, handler: CommandHandler){
        self.handlers.push(handler);
    }
    pub async fn handle_update(&self, update: &Update){
        println!("{:#?}", update);
    }
}

pub struct Updater {
    pub bot: Bot,
    pub base_endpoint: String,
    pub reqwest_client: reqwest::Client,
    pub dispatcher: Dispatcher
}

impl Updater {
    pub fn new(bot_token: &str) -> Self {
        Self {
            bot: Bot::new(bot_token.into()),
            base_endpoint: format!("https://api.telegram.org/bot{}/", bot_token).into(),
            reqwest_client: reqwest::Client::new(),
            dispatcher: Dispatcher::new()
        }
    }

    pub async fn start_polling(&self){
        let mut offset: i64 = 0;
        loop {
            let url = format!("{}{}?offset={}", self.base_endpoint, "getUpdates", offset+1);
            println!("Working 1");
            let result = self.reqwest_client.get(&url).send().await.unwrap().text().await.unwrap();
            println!("Working 2");
            let resp = serde_json::from_str::<Updates>(&result).unwrap();
            println!("Working 3");
            for update in resp.result.iter() {
                self.dispatcher.handle_update(update).await;
                offset = max(offset, update.update_id);
            }
            println!("Working 4");
        }
    }
}
