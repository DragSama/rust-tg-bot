use crate::types::Update;
use async_trait::async_trait;
use dyn_clone::{clone_trait_object, DynClone};

#[async_trait]
pub trait Handler: Send + Sync + DynClone {
    async fn check_update(&self, update: Update);
}

clone_trait_object!(Handler);

#[async_trait]
pub trait Dispatcher {
    async fn handle_update(&self, update: Update);
    fn add_handler(&mut self, handler: Box<dyn Handler>, group: Option<i32>);
}
