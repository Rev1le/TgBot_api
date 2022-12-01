use std::collections::HashMap;
use TgBot_api::{
    TGBotMethods,
    telegram_bot_methods::TelegramBotMethods,
    LongPullTelegramBot,
    InlineKeyboardMarkup
};

#[tokio::main]
async fn main()  {
    let bot = LongPullTelegramBot::new(
        "bot5654527036:AAHN8_qB8Ons8X_HSwbdvc2hBhpoUlu9zy4"
    );

    //let call_bot = CallbackTelegramBot::new("ee");



    let mut button_one = HashMap::new();
    button_one.insert("text", "hello");
    button_one.insert("callback_data", "test_one_button");

    let mut button_two = HashMap::new();
    button_two.insert("text", "bye");
    button_two.insert("callback_data", "test_two_button");

    let keyboar = InlineKeyboardMarkup {
        inline_keyboard: vec![vec![button_one, button_two]],
    };


    println!("{}", keyboar.keyboard_as_str());

    loop {
        //println!("{}", TelegramBot::get_me(bot.get_token()).await);


        // Для longPull бота
        for update in  bot.get_updates().await.unwrap() {
            println!("Обновление с id ={} : {:?}",  update.update_id, update.message);

            if let None = update.message {
                continue
            }


            if let Some(val) = &update.message.as_ref().unwrap().text {
                if val == "Hello" {
                    TGBotMethods::send_message(
                        bot.get_token(),
                        [
                            ("chat_id", &update.message.as_ref().unwrap().chat.id.to_string()),
                            ("text", &"Hello".to_string())
                        ],
                    ).await;
                }
            }
        }
    }
}
