# Rust Telegram Bot
Telegram Bot API wrapper, The design is inspired from [Python Telegram Bot](https://github.com/python-telegram-bot/python-telegram-bot).

## Note
This project is a discontinued because some genius deleted main repo

## Example
```rust
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
    let mut updater = Updater::new(&env!("TOKEN").to_string());
    updater.dispatcher.add_handler(Box::new(CommandHandler {
        command: "echo",
        func: echo,
    }));
    updater.start_polling().await?;
    Ok(())
}
```
