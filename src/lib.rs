pub mod bot;
pub mod weather_api;
pub mod state;
pub mod storage;
pub mod alerts;
pub mod scheduler;

use teloxide::{
    dispatching::{UpdateFilterExt, UpdateHandler},
    prelude::*,
};
use std::env;

/// Инициализирует все настройки бота
/// Должна вызываться и в реальном боте, и в тестере для обеспечения идентичной логики
pub fn initialize_bot() {
    if env::var("WEATHER_API_KEY").is_err() {
        dotenv::dotenv().ok();
    }
    
    if env::var("WEATHER_API_KEY").is_err() {
        panic!("WEATHER_API_KEY environment variable must be set");
    }
    
    pretty_env_logger::init();
    
    log::info!("Bot initialization completed");
}

pub fn handler_tree(shared_state: state::SharedState) -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    let shared_state_clone = shared_state.clone();
    let shared_state_clone2 = shared_state.clone();
    
    dptree::entry()
        .branch(
            Update::filter_message()
                .filter_command::<bot::Command>()
                .endpoint({
                    let state = shared_state_clone;
                    move |bot, msg, cmd| bot::answer(bot, msg, cmd, state.clone())
                })
        )
        .branch(
            Update::filter_callback_query()
                .endpoint({
                    let state = shared_state_clone2.clone();
                    move |bot, q| bot::callback_handler(bot, q, state.clone())
                })
        )
        .branch(
            Update::filter_message()
                .endpoint({
                    let state = shared_state.clone();
                    move |bot, msg| bot::message_handler(bot, msg, state.clone())
                })
        )
} 