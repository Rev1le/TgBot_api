use std::{
    fmt::Display
};
use serde_derive::Deserialize;
use crate::update::Update;

use super::TG_API;

#[derive(Debug, Deserialize, Clone)]
struct ResponseUpdate {
    ok: bool,
    result: Vec<Update>
}

pub struct LongPullTelegramBot {
    bot_token: String,
    last_update_id: std::cell::RefCell<i64>,
}

impl LongPullTelegramBot {

    pub fn new<T: Display>(bot_token: T) -> Self {
        LongPullTelegramBot {
            bot_token: bot_token.to_string(),
            last_update_id: std::cell::RefCell::new(0)
        }
    }

    pub fn get_token(&self) -> &str {
        &self.bot_token
    }

    pub async fn get_updates(&self) -> Result<Vec<Update>, ()> {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        let request = reqwest::get(
            format!("{}/{}/getUpdates?timeout=1&offset={}", TG_API, self.bot_token, *self.last_update_id.borrow()+1)
        ).await.unwrap().json::<ResponseUpdate>().await.unwrap();

        if let Some(last_update) = request.result.last() {
            *self.last_update_id.borrow_mut() = last_update.update_id;
        }

        Ok(request.result)
    }

    pub async fn get_updates_as_str(&self) -> String {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        let request = reqwest::get(
            format!("{}/{}/getUpdates?timeout=1&offset={}", TG_API, self.bot_token, *self.last_update_id.borrow()+1)
        ).await.unwrap().text().await.unwrap();

        if let Some(last_update) = serde_json::from_str::<ResponseUpdate>(&request).unwrap().result.last() {
            *self.last_update_id.borrow_mut() = last_update.update_id;
        }

        request
    }
}