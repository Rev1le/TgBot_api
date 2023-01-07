pub mod types;

pub mod bot_methods;
mod longpull_bot;

pub use longpull_bot::*;
pub use bot_methods::*;

// Telegram API link
const TG_API: &str = "https://api.telegram.org";
