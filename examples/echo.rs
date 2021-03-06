// Simple bot that has one command /echo, that replies with the text user sent
//
// To run this example use:
// cargo run --example echo
// Set "TOKEN" env var before that.

use rust_tg_bot::{
    bot::Bot, error::Result, handlers::CommandHandler, traits::dispatcher::Dispatcher,
    types::Update, updater::Updater,
};
use std::env;

async fn echo(update: Update, bot: Bot) -> Result<()> {
    let message = update.message.unwrap();
    bot.send_message(message.chat.id, message.text.unwrap())
        .send()
        .await?;
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

    // Here we add a new handler "CommandHandler" to dispatcher
    updater.dispatcher.add_handler(Box::new(CommandHandler {
        command: "echo",
        func: echo,
    }));
    // Starts listening for updates using long-polling method
    updater.start_polling().await?;
    Ok(())
}
