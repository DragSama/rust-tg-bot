use crate::types::Update;
use async_trait::async_trait;

#[async_trait]
pub trait Handler: Send + Sync {
    async fn check_update(&self, update: &Update);
}

#[async_trait]
pub trait Dispatcher {
    async fn handle_update(&self, update: Update);
    fn add_handler(&mut self, handler: Box<dyn Handler>, group: Option<i32>);
}
