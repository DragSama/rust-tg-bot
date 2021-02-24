extern crate reqwest;
pub mod types
use types::update::Update

pub struct Updater {
    pub bot_token: String,
    pub endpoint: String,
    pub reqwest_client: reqwest::Client
}

impl Updater {
    pub fn new(bot_token: &str) -> Self{
        Self {
            bot_token: bot_token.into(),
            endpoint: "https://api.telegram.org/bot123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11/".into(),
            reqwest_client: reqwest::Client::new()
        }
    }

    // fn handle_update(){}
}
// fn main() {
//     println!("Hello, world!");
// }
