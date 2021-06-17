use crate::{error::Result, methods::SendMessage};
use reqwest;

#[derive(Clone)]
pub struct Bot {
    pub token: String,
    base_endpoint: String,
    reqwest_client: reqwest::Client,
}

impl Bot {
    pub fn new(bot_token: &str) -> Self {
        Self {
            token: bot_token.into(),
            base_endpoint: format!("https://api.telegram.org/bot{}/", bot_token).into(),
            reqwest_client: reqwest::Client::new(),
        }
    }
    pub(crate) async fn send(
        &self,
        method: &str,
        args: Option<String>,
    ) -> Result<reqwest::Response> {
        let url = format!("{}{}", self.base_endpoint, method);
        let response = if args.is_some() {
            let form: serde_json::Value = serde_json::from_str(&args.unwrap())?;
            self.reqwest_client.post(url).form(&form).send().await?
        } else {
            self.reqwest_client.post(url).send().await?
        };
        Ok(response)
    }

	pub async fn send_message(&self, chat_id: i64, text: String) -> SendMessage<'_> {
		SendMessage::new(self, chat_id, text)
	}
}
