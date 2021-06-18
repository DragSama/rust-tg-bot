use crate::{bot::Bot, traits::dispatcher::Handler, types::Update};
use async_trait::async_trait;

pub struct RawHandler<F>
where
    F: std::future::Future + Send + 'static,
{
    pub func: fn(Update, Bot) -> F,
}

impl<F> Clone for RawHandler<F>
where
    F: std::future::Future + Send + 'static,
{
    fn clone(&self) -> Self {
        Self {
            func: self.func.clone(),
        }
    }
}

#[async_trait]
impl<F> Handler for RawHandler<F>
where
    F: std::future::Future + Send + 'static,
{
    async fn check_update(&self, update: Update, bot: Bot) {
        (self.func)(update, bot).await;
    }
}
