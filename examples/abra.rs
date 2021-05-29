use rust_tg_bot::{Updater, CommandHandler, types::update::Update, traits::dispatcher::Dispatcher};

use std::env;

async fn echo(update: Update){
     let message = update.message.unwrap();
     println!("Update from chat: {}", message.chat.id);
}

#[tokio::main]
async fn main(){
    let mut bot = Updater::new(&env!("TOKEN").to_string());
    bot.dispatcher.add_handler(Box::new(CommandHandler{command: "echo".to_string(), func: echo}), None);
    bot.start_polling().await;
}
