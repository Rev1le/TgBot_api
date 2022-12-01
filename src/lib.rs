use std::collections::HashMap;
use async_trait::async_trait;
use serde_derive::Serialize;
use serde_json::json;

mod update;
use update::Update;

mod telegram_longpull_bot;
pub use telegram_longpull_bot::LongPullTelegramBot;
//pub mod callback_bot;
pub mod telegram_bot_methods;

const TG_API: &str = "https://api.telegram.org";

pub struct TGBotMethods;

#[async_trait]
impl telegram_bot_methods::TelegramBotMethods for TGBotMethods {}

#[derive(Serialize)]
pub struct InlineKeyboardMarkup<'a> {
    pub inline_keyboard: Vec<Vec<HashMap<&'a str, &'a str>>>
}

impl<'a> InlineKeyboardMarkup<'a> {
    pub fn builder() -> Self {
        InlineKeyboardMarkup {
            inline_keyboard: vec![vec![]],
        }
    }

    pub fn push_line(&mut self, line: Vec<HashMap<&'a str, &'a str>>) {
        self.inline_keyboard.push(line)
    }

    pub fn keyboard_as_str(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
