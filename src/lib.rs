#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate reqwest;
extern crate serde_json;

pub mod error;
pub mod methods;
pub mod traits;
pub mod types;

use crate::{
    error::Error,
    methods::GetUpdates,
    traits::dispatcher::{Dispatcher as dp_trait, Handler},
    types::{Update, Updates},
};
use async_trait::async_trait;
use std::cmp::max;

pub struct Bot {
    pub token: String,
    base_endpoint: String,
    reqwest_client: reqwest::Client,
}

impl Bot {
    pub fn new(bot_token: &str) -> Self {
        Self {
            token: bot_token.into(),
            base_endpoint: format!("https://api.telegram.org/bot{}/", bot_token).into(),
            reqwest_client: reqwest::Client::new(),
        }
    }
    async fn send(&self, method: &str, body: String) -> Result<reqwest::Response, Error> {
        let url = format!("{}{}", self.base_endpoint, method);
        let response = self.reqwest_client.post(url).body(body).send().await?;
        Ok(response)
    }
}

pub struct CommandHandler<'a, F>
where
    F: std::future::Future + Send + 'static,
{
    pub command: &'a str,
    pub func: fn(Update) -> F,
}

impl<F> Clone for CommandHandler<'_, F>
where
    F: std::future::Future + Send + 'static,
{
    fn clone(&self) -> Self {
        Self {
            command: self.command.clone(),
            func: self.func.clone(),
        }
    }
}

#[async_trait]
impl<F> Handler for CommandHandler<'_, F>
where
    F: std::future::Future + Send + 'static,
{
    async fn check_update(&self, update: Update) {
        let cloned_update = update.clone();
        if update.message.is_some() {
            let message = update.message.unwrap();
            if message.text.is_some()
                && message
                    .text
                    .unwrap()
                    .starts_with(&format!("{}{}", '/', self.command))
            {
                (self.func)(cloned_update).await;
            }
        }
    }
}

#[derive(Clone)]
pub struct Dispatcher {
    pub handlers: Vec<Box<dyn Handler>>,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }
}

#[async_trait]
impl dp_trait for Dispatcher {
    fn add_handler(&mut self, handler: Box<dyn Handler>, group: Option<i32>) {
        self.handlers.push(handler);
    }
    async fn handle_update(&self, update: Update) {
        for handler in self.handlers.clone() {
            let up = update.clone();
            tokio::spawn(async move {
                handler.check_update(up).await;
            });
        }
    }
}

pub struct Updater {
    pub bot: Bot,
    base_endpoint: String,
    reqwest_client: reqwest::Client,
    pub dispatcher: Dispatcher,
}

impl Updater {
    pub fn new(bot_token: &str) -> Self {
        Self {
            bot: Bot::new(bot_token.into()),
            base_endpoint: format!("https://api.telegram.org/bot{}/", bot_token).into(),
            reqwest_client: reqwest::Client::new(),
            dispatcher: Dispatcher::new(),
        }
    }

    pub async fn start_polling(&self) -> Result<(), Error> {
        let mut offset: i64 = 0;
        loop {
            let text = self
                .bot
                .send(
                    "getUpdates",
                    serde_json::to_string(&GetUpdates {
                        offset: Some(offset + 1),
                        limit: None,
                        timeout: Some(10),
                        allowed_updates: None,
                    })
                    .unwrap(),
                )
                .await?
                .text()
                .await?;
            // println!("{:#?}", response);
            // let url = format!("{}{}?offset={}?timeout=10", self.base_endpoint, "getUpdates", offset+1);
            // let result = self.reqwest_client.get(&url).send().await.unwrap().text().await.unwrap();
            let resp = serde_json::from_str::<Updates>(&text).unwrap();
            println!("offset: {}", offset);
            for update in resp.result {
                offset = max(offset, update.update_id);
                self.dispatcher.handle_update(update).await;
            }
        }
    }
}
