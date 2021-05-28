use rust_tg_bot::{Updater, CommandHandler, types::update::Update, traits::dispatcher::Dispatcher};

use std::env;

async fn echo(&Update){}

#[tokio::main]
async fn main(){
    let bot = Updater::new(&env!("TOKEN").to_string());
    bot.dispatcher.add_handler(CommandHandler{command: "echo", func: echo});
    bot.start_polling().await;
}
