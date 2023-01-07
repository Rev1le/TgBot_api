use std::collections::HashMap;
use serde::ser::SerializeStruct;
use serde_derive::Serialize;


pub struct InlineKeyboardButton {
    pub text: String,
    pub url: Option<String>,
    pub callback_data: Option<String>,
    //web_app:
    pub login_url: Option<String>,
    pub switch_inline_query: Option<String>,
    pub switch_inline_query_current_chat: Option<String>,
    //callback_game
    pub pay: Option<bool>
}

impl serde::ser::Serialize for InlineKeyboardButton {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S:  serde::ser::Serializer,
    {
        let mut s = serializer.serialize_struct("InlineKeyboardButton", 7)?;

        s.serialize_field("text", &self.text)?;

        if let Some(url) = &self.url {
            s.serialize_field("url", url)?;
        }
        if let Some(callback_data) = &self.callback_data {
            s.serialize_field("callback_data", callback_data)?;
        }
        if let Some(login_url) = &self.login_url {
            s.serialize_field("login_url", login_url)?;
        }
        if let Some(switch_inline_query) = &self.switch_inline_query {
            s.serialize_field("switch_inline_query", switch_inline_query)?;
        }
        if let Some(switch_inline_query_current_chat) = &self.switch_inline_query_current_chat {
            s.serialize_field("switch_inline_query_current_chat", switch_inline_query_current_chat)?;
        }
        if let Some(pay) = &self.pay {
            s.serialize_field("pay", pay)?;
        }
        s.end()
    }
}

impl InlineKeyboardButton {
    pub fn new(
        text: String,
        url: Option<String>,
        callback_data: Option<String>,
        login_url: Option<String>,
        switch_inline_query: Option<String>,
        switch_inline_query_current_chat: Option<String>,
        pay: Option<bool>) -> Self
    {
        InlineKeyboardButton {
            text,
            url,
            callback_data,
            login_url,
            switch_inline_query,
            switch_inline_query_current_chat,
            pay,
        }
    }
}


#[derive(Serialize)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>
}

impl<'a> InlineKeyboardMarkup {
    pub fn builder() -> Self {
        InlineKeyboardMarkup {
            inline_keyboard: vec![],
        }
    }

    pub fn push_line(&mut self, line: Vec<InlineKeyboardButton>) {
        self.inline_keyboard.push(line)
    }

    pub fn keyboard_as_str(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
