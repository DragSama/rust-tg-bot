use rust_tg_bot::{
    bot::Bot, error::Result, handlers::CommandHandler, traits::dispatcher::Dispatcher,
    types::Update, updater::Updater,
};
use std::env;

async fn echo(update: Update, bot: Bot) -> Result<()> {
    let message = update.message.unwrap();
    bot.send_message(message.chat.id, message.text.unwrap())
        .await
        .send()
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut bot = Updater::new(&env!("TOKEN").to_string());
    bot.dispatcher.add_handler(Box::new(CommandHandler {
        command: "echo",
        func: echo,
    }));
    bot.start_polling().await?;
    Ok(())
}
