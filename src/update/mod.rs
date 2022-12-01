use serde::Deserializer;
use serde_derive::Deserialize;
use serde_json::Value;

mod message;
mod chat;

#[derive(Debug, Deserialize, Clone)]
pub struct Update {
    pub update_id: i64,
    pub message: Option<message::Message>,
    pub edited_message: Option<message::Message>,
    pub channel_post: Option<message::Message>,
    pub edited_channel_post: Option<message::Message>,
}
