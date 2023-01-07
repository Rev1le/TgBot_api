use serde_derive::{Deserialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Clone)]
pub struct Chat {
    pub id: i64,
    pub type_chat: Option<String>,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_forum: Option<bool>,
    pub photo: Option<Value>,
    pub active_usernames: Option<Value>,
    pub emoji_status_custom_emoji_id: Option<Value>,
    pub bio: Option<Value>,
    pub has_private_forwards: Option<Value>,
    pub has_restricted_voice_and_video_messages: Option<Value>,
    pub join_to_send_messages: Option<Value>,
    pub join_by_request: Option<Value>,
    pub description: Option<Value>,
    pub invite_link: Option<Value>,
    pub pinned_message: Option<Value>,
    pub permissions: Option<Value>,
    pub slow_mode_delay: Option<Value>,
    pub message_auto_delete_time: Option<Value>,
    pub has_protected_content: Option<Value>,
    pub sticker_set_name: Option<Value>,
    pub can_set_sticker_set: Option<Value>,
    pub linked_chat_id: Option<Value>,
    pub location: Option<Value>
}