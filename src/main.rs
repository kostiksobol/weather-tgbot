use teloxide::prelude::*;
use weather_tgbot::{initialize_bot, handler_tree, state::create_shared_state};

#[tokio::main]
async fn main() {
    // Используем централизованную инициализацию
    initialize_bot();
    log::info!("Starting weather bot...");

    let bot = Bot::from_env();
    let shared_state = create_shared_state();
    
    Dispatcher::builder(bot, handler_tree(shared_state))
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
