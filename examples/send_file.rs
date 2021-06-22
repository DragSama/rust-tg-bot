use rust_tg_bot::{bot::Bot, types::InputFile, error::Result};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let bot = Bot::new(&env!("TOKEN").to_string());
    let user: i64 = env!("USER").to_string().parse().expect("Invalid user id");
    bot.send_animation(user, InputFile::Url("https://media0.giphy.com/media/ya4eevXU490Iw/giphy.gif".into()))
        .send()
        .await?;
    Ok(())
}
