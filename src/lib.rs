#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate reqwest;
extern crate serde_json;

pub mod types;
pub mod methods;
pub mod traits;

use crate::{types::{Update, Updates}, traits::dispatcher::{Dispatcher as dp_trait, Handler}};
use async_trait::async_trait;
use std::cmp::max;

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

pub struct CommandHandler<F> 
where
    F: std::future::Future + Send
{
    pub command: String,
    pub func: fn(Update) -> F
}

#[async_trait]
impl<F> Handler for CommandHandler<F> 
where
    F: std::future::Future + Send
{
    async fn check_update(&self, update: Update){
        let message = match update.message {
            Some(ref m) => m,
            _ => return ()
        };
        if message.text.as_ref().unwrap_or(&"".to_string()).starts_with(&self.command){
            (self.func)(update).await;
        }
    }
}

pub struct Dispatcher {
    pub handlers: Vec<Box<dyn Handler>>
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new()
        }
    }
}

#[async_trait]
impl dp_trait for Dispatcher {
    fn add_handler(&mut self, handler: Box<dyn Handler>, group: Option<i32>){
        self.handlers.push(handler);
    }
    async fn handle_update(&self, update: Update){
        for handler in self.handlers.iter(){
	handler.check_update(update.clone());
        }
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
            let url = format!("{}{}?offset={}?timeout=10", self.base_endpoint, "getUpdates", offset+1);
            let result = self.reqwest_client.get(&url).send().await.unwrap().text().await.unwrap();
            let resp = serde_json::from_str::<Updates>(&result).unwrap();
            for update in resp.result {
	offset = max(offset, update.update_id);
                self.dispatcher.handle_update(update).await;
            }
        }
    }
}
