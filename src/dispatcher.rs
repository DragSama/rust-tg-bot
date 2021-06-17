use crate::{
    traits::dispatcher::{Dispatcher as DispatcherTrait, Handler},
    types::Update,
	bot::Bot
};
use async_trait::async_trait;

#[derive(Clone)]
pub struct Dispatcher {
    pub handlers: Vec<Box<dyn Handler>>,
	bot: Bot
}

impl Dispatcher {
    pub fn new(bot: Bot) -> Self {
        Self {
            handlers: Vec::new(),
			bot: bot
        }
    }
}

#[async_trait]
impl DispatcherTrait for Dispatcher {
	/// Adds a handler, A handler is valid If It implements Handler trait.
    fn add_handler(&mut self, handler: Box<dyn Handler>) {
        self.handlers.push(handler);
    }

    async fn handle_update(&self, update: Update) {
        for handler in self.handlers.clone() {
            let up = update.clone();
			let bot = self.bot.clone();
            tokio::spawn(async move {
                handler.check_update(up, bot).await;
            });
        }
    }
}
