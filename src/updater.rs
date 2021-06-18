use crate::{
    bot::Bot, dispatcher::Dispatcher, error::Result, methods::GetUpdates,
    traits::dispatcher::Dispatcher as DispatcherTrait,
};
use std::cmp::max;

pub struct Updater {
    pub bot: Bot,
    pub dispatcher: Dispatcher,
}

impl Updater {
    pub fn new(bot_token: &str) -> Self {
        let bot = Bot::new(bot_token.into());
        Self {
            dispatcher: Dispatcher::new(bot.clone()),
            bot: bot,
        }
    }

    pub async fn start_polling(&self) -> Result<()> {
        let mut offset: i64 = 0;
        loop {
            let updates = self.bot.get_updates()
                .offset(offset + 1)
                .timeout(10)
                .send()
                .await?;
            for update in updates.result {
                offset = max(offset, update.update_id);
                self.dispatcher.handle_update(update).await;
            }
        }
    }
}
