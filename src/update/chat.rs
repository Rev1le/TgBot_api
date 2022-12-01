use super::{Deserialize, Deserializer};

#[derive(Debug, Deserialize, Clone)]
pub struct Chat {
    pub id: i64,
    pub type_chat: Option<String>,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_forum: Option<bool>,
    pub photo: Option<super::Value>,
    pub active_usernames: Option<super::Value>,
    pub emoji_status_custom_emoji_id: Option<super::Value>,
    pub bio: Option<super::Value>,
    pub has_private_forwards: Option<super::Value>,
    pub has_restricted_voice_and_video_messages: Option<super::Value>,
    pub join_to_send_messages: Option<super::Value>,
    pub join_by_request: Option<super::Value>,
    pub description: Option<super::Value>,
    pub invite_link: Option<super::Value>,
    pub pinned_message: Option<super::Value>,
    pub permissions: Option<super::Value>,
    pub slow_mode_delay: Option<super::Value>,
    pub message_auto_delete_time: Option<super::Value>,
    pub has_protected_content: Option<super::Value>,
    pub sticker_set_name: Option<super::Value>,
    pub can_set_sticker_set: Option<super::Value>,
    pub linked_chat_id: Option<super::Value>,
    pub location: Option<super::Value>
}