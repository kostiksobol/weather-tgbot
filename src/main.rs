use teloxide::prelude::*;
use weather_tgbot::{initialize_bot, handler_tree, state::create_shared_state, storage::Storage, scheduler::AlertScheduler};

#[tokio::main]
async fn main() {
    // Используем централизованную инициализацию
    initialize_bot();
    log::info!("Starting weather bot...");

    let bot = Bot::from_env();
    
    // Создаем Storage и загружаем данные
    let storage = match Storage::new() {
        Ok(storage) => storage,
        Err(e) => {
            log::error!("Failed to initialize storage: {}", e);
            return;
        }
    };
    
    // Загружаем существующие данные пользователей
    let shared_state = match storage.load_all_users() {
        Ok(loaded_data) => {
            use weather_tgbot::state::create_shared_state_with_data;
            create_shared_state_with_data(storage.clone(), loaded_data)
        },
        Err(e) => {
            log::error!("Failed to load user data: {}", e);
            match create_shared_state() {
                Ok(state) => state,
                Err(e) => {
                    log::error!("Failed to create shared state: {}", e);
                    return;
                }
            }
        }
    };
    
    // Очищаем данные заблокированных пользователей при запуске
    log::info!("Cleaning up blocked users...");
    if let Err(e) = shared_state.storage.cleanup_blocked_users(&bot).await {
        log::warn!("Failed to cleanup blocked users: {}", e);
    }
    
    // Показываем статистику
    if let Err(e) = shared_state.storage.stats() {
        log::warn!("Failed to get storage stats: {}", e);
    }
    
    log::info!("Weather bot started successfully!");
    
    // Запускаем scheduler для проверки алертов в фоновом режиме
    let scheduler = AlertScheduler::new(bot.clone(), shared_state.clone());
    tokio::spawn(async move {
        scheduler.start().await;
    });
    
    log::info!("Alert scheduler started");
    
    Dispatcher::builder(bot, handler_tree(shared_state))
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
