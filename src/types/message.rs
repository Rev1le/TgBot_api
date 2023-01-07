use super::chat;
use serde_derive::{Deserialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Clone)]
pub struct Message {
    pub message_id: i64,
    pub message_thread_id: Option<i64>,
    pub from: Option<Value>,
    pub sender_chat: Option<chat::Chat>,
    pub date: i64,
    pub chat: chat::Chat,
    pub text: Option<String>
}