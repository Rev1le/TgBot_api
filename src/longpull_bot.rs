use std::fmt::Display;
use reqwest::Response;
use serde_derive::Deserialize;

use super::bot_methods::TelegramBotMethods;
use super::types::update;
use super::TG_API;

#[derive(Clone, Deserialize)]
struct ResponseUpdate {
    ok: bool,
    result: Vec<update::Update>
}

pub struct LongPullTelBot {
    bot_token: String,
    last_update_id: std::cell::Cell<i64>,
}

impl TelegramBotMethods for LongPullTelBot {}

impl LongPullTelBot {

    pub fn new<T: Display>(bot_token: T) -> Self {
        LongPullTelBot {
            bot_token: bot_token.to_string(),
            last_update_id: std::cell::Cell::new(0)
        }
    }

    pub fn get_token(&self) -> &str {
        &self.bot_token
    }

    async fn request_get_updates(&self, timeout: u64) -> reqwest::Result<Response> {
        tokio::time::sleep(tokio::time::Duration::from_secs(timeout)).await;

        reqwest::get(
            format!(
                "{}/{}/getUpdates?timeout={}&offset={}",
                TG_API,
                self.bot_token,
                timeout,
                self.last_update_id.get()+1
            )
        ).await
    }

    pub async fn get_updates(&self) -> Result<Vec<update::Update>, ()> {

        let response_result = self.request_get_updates(1).await;

        if let Ok(response) = response_result {

            let response_update = response.json::<ResponseUpdate>().await.unwrap();

            if let Some(last_update) = response_update.result.last() {
                self.last_update_id.set(last_update.update_id);
            }

            return Ok(response_update.result)
        }

        Err(())
    }

    pub async fn get_updates_as_str(&self) -> String {

        let response = self.request_get_updates(1)
            .await
            .expect("Ошибка запроса к Telegram API");
        let body_text = response.text().await.unwrap();
        let updates =  serde_json::from_str::<ResponseUpdate>(&body_text)
            .unwrap()
            .result;

        let last_update_option = updates.last();

        if let Some(last_update) = last_update_option {
            self.last_update_id.set(last_update.update_id);
        }

        body_text
    }
}