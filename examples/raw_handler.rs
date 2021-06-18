// Simple bot that just prints the update is recieves
//
// To run this example use:
// cargo run --example echo
// Set "TOKEN" env var before that.

use rust_tg_bot::{
    bot::Bot, error::Result, handlers::RawHandler, traits::dispatcher::Dispatcher,
    types::Update, updater::Updater,
};
use std::env;

async fn raw_handler(update: Update, bot: Bot) -> Result<()> {
    println!("Got update: {:#?}", update);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Creates a new updater and passes it TOKEN env var
    // By default Updater will create own Dispatcher and Bot, Accessible as updater.dispatcher and updater.bot respectively
    let mut updater = Updater::new(&env!("TOKEN").to_string());

    // A quick explanation for what these do:
    // Updater - gets the update
    // Dispatcher - maintains a list of handlers, is called by Updater
    // Handler - checks update to see if its valid and then call the function

    // Raw handler will call function for every update it gets
    updater.dispatcher.add_handler(Box::new(RawHandler { func: raw_handler }));
    // Starts listening for updates using long-polling method
    updater.start_polling().await?;
    Ok(())
}
