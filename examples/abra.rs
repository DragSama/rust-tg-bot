use rust_tg_bot::{Updater, CommandHandler, types::update::Update, traits::dispatcher::Dispatcher};

use std::env;

async fn echo(update: &Update) -> () {}

#[tokio::main]
async fn main(){
    let bot = Updater::new(&env!("TOKEN").to_string());
    bot.dispatcher.add_handler(Box::new(CommandHandler{command: "echo".to_string(), func: echo}), None);
    bot.start_polling().await;
}
