use std::borrow::Borrow;
use serde_json::Value;
use async_trait::async_trait;
use url::Url;

const TG_API: &str = "https://api.telegram.org";

#[async_trait]
pub trait TelegramBotMethods {

    async fn send_api_method<I, K, V>(method_name: &str, token: &str, params: I) -> bool
        where
            I: IntoIterator + Send,
            <I as IntoIterator>::Item: Borrow<(K, V)>,
            K: AsRef<str>,
            V: AsRef<str>
    {
        let url = Url::parse_with_params(
            &format!("{}/{}/{}", TG_API, token, method_name),
            params
        );

        reqwest::get(url.unwrap()).await.is_ok()
    }

    // A simple method for testing your bot's authentication token.
    // Requires no parameters. Returns basic information about the bot in form of a User object.
    async fn get_me(token: &str) -> Value {
        reqwest::get(
            &format!("{}/{}/getMe", TG_API, token)
        ).await.unwrap().json::<Value>().await.unwrap()
    }

    // Use this method to send text messages. On success, the sent Message is returned.
    async fn send_message<I, K, V>(token: &str, params: I) -> bool
        where
            I: IntoIterator + Send,
            <I as IntoIterator>::Item: Borrow<(K, V)>,
            K: AsRef<str>,
            V: AsRef<str>
    {
        Self::send_api_method("sendMessage", token, params).await
    }

    // Use this method to delete a message, including service messages, with the following limitations:
    // - A message can only be deleted if it was sent less than 48 hours ago.
    // - Service messages about a supergroup, channel, or forum topic creation can't be deleted.
    // - A dice message in a private chat can only be deleted if it was sent more than 24 hours ago.
    // - Bots can delete outgoing messages in private chats, groups, and supergroups.
    // - Bots can delete incoming messages in private chats.
    // - Bots granted can_post_messages permissions can delete outgoing messages in channels.
    // - If the bot is an administrator of a group, it can delete any message there.
    // - If the bot has can_delete_messages permission in a supergroup or a channel, it can delete any message there.
    // Returns True on success.
    async fn delete_message<I, K, V>(token: &str, params: I) -> bool
        where
            I: IntoIterator + Send,
            <I as IntoIterator>::Item: Borrow<(K, V)>,
            K: AsRef<str>,
            V: AsRef<str>
    {
        Self::send_api_method("deleteMessage", token, params).await
    }

    // Use this method to copy messages of any kind.
    // Service messages and invoice messages can't be copied.
    // A quiz poll can be copied only if the value of the field correct_option_id is known to the bot.
    // The method is analogous to the method forwardMessage,
    // but the copied message doesn't have a link to the original message.
    // Returns the MessageId of the sent message on success.
    async fn copy_message<I, K, V>(token: &str, params: I) -> bool
        where
            I: IntoIterator + Send,
            <I as IntoIterator>::Item: Borrow<(K, V)>,
            K: AsRef<str>,
            V: AsRef<str>
    {
        Self::send_api_method("copyMessage", token, params).await
    }

    // Use this method to get the current list of the bot's commands for the given scope and user language.
    // Returns an Array of BotCommand objects. If commands aren't set, an empty list is returned.
    async fn set_my_commands<I, K, V>(token: &str, params: I) -> bool
        where
            I: IntoIterator + Send,
            <I as IntoIterator>::Item: Borrow<(K, V)>,
            K: AsRef<str>, V: AsRef<str>
    {
        Self::send_api_method("setMyCommands", token, params).await
    }
}
