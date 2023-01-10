use serde_json::Value;
use async_trait::async_trait;

use super::TG_API;

#[async_trait]
pub trait TelegramBotMethods
{
    async fn send_api_method<I, K, V>(method_name: &str, token: &str, params: I) -> Result<Value, String>
        where
            I:IntoIterator + Send,
            I::Item: std::borrow::Borrow<(K, V)>,
            K: AsRef<str>,
            V : AsRef<str>,
    {
        let url = url::Url::parse_with_params(
            &format!("{}/{}/{}", TG_API, token, method_name),
            params
        );

        if let &Err(e) = &url {
            return Err(format!("{}", e))
        }

        let response_result = reqwest::get(url.unwrap()).await;

        if let Ok(response) = response_result {
            return Ok(response.json::<Value>().await.unwrap());
        }
        Err(String::from("Response error"))
    }
}
