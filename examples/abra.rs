extern crate rust_tg_bot;

use std::env;

#[tokio::main]
async fn main(){
    let bot = rust_tg_bot::Updater::new(&env!("TOKEN").to_string());
    bot.start_polling().await;
}
