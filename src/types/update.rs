use serde::Deserializer;
use serde_derive::Deserialize;
use serde_json::Value;
use super::message;
use super::chat;

#[derive(Debug, Deserialize, Clone)]
pub struct Update {
    pub update_id: i64,
    pub message: Option<message::Message>,
    pub edited_message: Option<message::Message>,
    pub channel_post: Option<message::Message>,
    pub edited_channel_post: Option<message::Message>,
    pub inline_query: Option<Value>,
    pub chosen_inline_result: Option<Value>,
    pub callback_query: Option<Value>,
    pub shipping_query: Option<Value>,
    pub pre_checkout_query: Option<Value>,
    pub poll: Option<Value>,
    pub poll_answer: Option<Value>,
    pub my_chat_member: Option<Value>,
    pub chat_member: Option<Value>,
    pub chat_join_request: Option<Value>
}
