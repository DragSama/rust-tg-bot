extern crate reqwest;
extern crate serde_json;

pub mod types;
use types::update::Update;

pub struct Updater {
    pub bot_token: String,
    pub base_endpoint: String,
    pub reqwest_client: reqwest::Client
}

impl Updater {
    pub fn new(bot_token: &str) -> Self{
        Self {
            bot_token: bot_token.into(),
            base_endpoint: format!("https://api.telegram.org/bot{}/", bot_token).into(),
            reqwest_client: reqwest::Client::new()
        }
    }
    async fn handle_update(self, update: Update){
        println!("Got update: {:#?}", update)
    }
    async fn start_polling(self){
        loop {
            let result = self.reqwest_client.get(&format!("{}/{}", self.base_endpoint, "getUpdates")).send().await.unwrap().text().await.unwrap();
            self.handle_update(serde_json::from_str::<Update>(&result)).await;
        }
    }
}
// fn main() {
//     println!("Hello, world!");
// }
