use std::fmt::Display;
use std::net::{Ipv4Addr};
use super::TG_API;

pub struct CallbackTelegramBot {
    webhook_url: String,

}

impl CallbackTelegramBot {

    pub fn new<T: Display>(webhook_url: T) -> Self {
        CallbackTelegramBot {
            webhook_url: webhook_url.to_string(),
        }
    }

    pub fn get_webhook_url(&self) -> &str {
        &self.webhook_url
    }

    pub async fn get_updates(&self, ip: Ipv4Addr, port: usize) -> Vec::<tokio::net::TcpStream> {
        let mut res = Vec::<tokio::net::TcpStream>::new();
        let listener = tokio::net::TcpListener::bind(format!("{}:{}", ip.to_string(), port.to_string())).await.unwrap();

        if let (socket, _) =  listener.accept().await.unwrap() {
            res.push(socket);
        }

        res
    }
}