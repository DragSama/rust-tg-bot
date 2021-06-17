use crate::{bot::Bot, traits::dispatcher::Handler, types::Update};
use async_trait::async_trait;

pub struct CommandHandler<'a, F>
where
    F: std::future::Future + Send + 'static,
{
    pub command: &'a str,
    pub func: fn(Update, Bot) -> F,
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
    async fn check_update(&self, update: Update, bot: Bot) {
        let cloned_update = update.clone();
        if update.message.is_some() {
            let message = update.message.unwrap();
            if message.text.is_some()
                && message
                    .text
                    .unwrap()
                    .starts_with(&format!("{}{}", '/', self.command))
            {
                (self.func)(cloned_update, bot).await;
            }
        }
    }
}
