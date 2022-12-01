use super::chat;
use super::{Deserialize, Deserializer};

#[derive(Debug, Deserialize, Clone)]
pub struct Message {
    pub message_id: i64,
    pub message_thread_id: Option<i64>,
    pub from: Option<super::Value>,
    pub sender_chat: Option<chat::Chat>,
    pub date: i64,
    pub chat: chat::Chat,
    pub text: Option<String>
}