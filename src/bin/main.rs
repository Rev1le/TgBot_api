use TgBot_api::{LongPullTelBot, TelegramBotMethods};

struct SendingBot;

impl TelegramBotMethods for SendingBot {}

#[tokio::main]
async fn main() {

    // Созжание бота для полчения обновлений
    let bot = LongPullTelBot::new(
        "bot5654527036:AAHN8_qB8Ons8X_HSwbdvc2hBhpoUlu9zy4"
    );

    // Создание клавиатуры
    let mut keyboard = TgBot_api::types::inline_keyboard::InlineKeyboardMarkup::builder();

    let button_one = TgBot_api::types::inline_keyboard::InlineKeyboardButton::new(
        "Hello".to_string(),
        None,
        Some("test_one_button".to_string()),
        None,
        None,
        None,
        None
    );

    let button_two = TgBot_api::types::inline_keyboard::InlineKeyboardButton::new(
        "Nye".to_string(),
        None,
        Some("test_two_button".to_string()),
        None,
        None,
        None,
        None
    );

    keyboard.push_line(vec![button_one, button_two]);

    // Для longPull бота
    loop {

        // Получение обновлений в виде строки
        println!("{}", bot.get_updates_as_str().await);

        // Получение обновлений в виде структуры
        for update in bot.get_updates(1).await.unwrap() {
            println!("Обновление с id ={} : {:?}", update.update_id, update);

            if let None = update.message {
                continue
            }

            if let Some(val) = &update.message.as_ref().unwrap().text {
                if val == "Hello" {
                    println!("Bot say Hello!");

                    let response = SendingBot::send_api_method(
                        "sendMessage",
                        bot.get_token(),
                        [
                            ("chat_id", &update.message.as_ref().unwrap().chat.id.to_string()),
                            ("text", &"Hello too".to_string()),
                            ("reply_markup", &keyboard.keyboard_as_str()),
                        ]
                    ).await;

                    println!("{:?}", response);
                }
            }


        }
    }
}
