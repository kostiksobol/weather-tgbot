use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::BotCommands,
};
use crate::{weather_api, state::{SharedState, get_user_data, update_user_data}};

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    #[command(description = "Display this text.")]
    Help,
    #[command(description = "Start the bot.")]
    Start,
}

pub async fn answer(bot: Bot, msg: Message, cmd: Command, _state: SharedState) -> HandlerResult {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Start => {
            let keyboard = make_main_menu_keyboard();
            bot.send_message(msg.chat.id, "Welcome! Please choose an option:")
                .reply_markup(keyboard)
                .await?
        }
    };

    Ok(())
}

pub async fn callback_handler(bot: Bot, q: CallbackQuery, state: SharedState) -> HandlerResult {
    // Answer the callback query first
    bot.answer_callback_query(q.id).await?;

    // Get the data from the callback
    if let Some(data) = q.data {
        if let Some(message) = q.message {
            let chat_id = message.chat().id;
            
            match data.as_str() {
                "get_weather" => {
                    // Set user state to waiting for city input
                    update_user_data(&state, chat_id, |user_data| {
                        user_data.waiting_for_city = true;
                    });
                    
                    bot.send_message(chat_id, "Please enter the name of the city you want to get weather for:")
                        .await?;
                }
                "my_towns" => {
                    let keyboard = make_my_towns_keyboard(&state, chat_id);
                    bot.send_message(chat_id, "Manage your towns:")
                        .reply_markup(keyboard)
                        .await?;
                }
                "set_home_town" => {
                    update_user_data(&state, chat_id, |user_data| {
                        user_data.waiting_for_home_town = true;
                    });
                    
                    bot.send_message(chat_id, "Please enter the name of your home town:")
                        .await?;
                }
                "add_interested_town" => {
                    update_user_data(&state, chat_id, |user_data| {
                        user_data.waiting_for_interested_town = true;
                    });
                    
                    bot.send_message(chat_id, "Please enter the name of the town you're interested in:")
                        .await?;
                }
                "remove_interested_town" => {
                    let user_data = get_user_data(&state, chat_id);
                    if user_data.interested_towns.is_empty() {
                        bot.send_message(chat_id, "You don't have any interested towns to remove.")
                            .await?;
                    } else {
                        let keyboard = make_remove_towns_keyboard(&state, chat_id);
                        bot.send_message(chat_id, "Select a town to remove:")
                            .reply_markup(keyboard)
                            .await?;
                    }
                }
                "view_home_weather" => {
                    let user_data = get_user_data(&state, chat_id);
                    if let Some(home_town) = &user_data.home_town {
                        // Send "typing" action while fetching weather
                        bot.send_chat_action(chat_id, teloxide::types::ChatAction::Typing).await?;
                        
                        match weather_api::get_current_weather(home_town).await {
                            Ok(weather) => {
                                let weather_message = weather_api::format_current_weather(&weather);
                                bot.send_message(chat_id, weather_message)
                                    .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                                    .await?;
                            }
                            Err(e) => {
                                bot.send_message(chat_id, format!("Sorry, I couldn't get the weather for your home town '{}'. Error: {}", home_town, e))
                                    .await?;
                            }
                        }
                    } else {
                        bot.send_message(chat_id, "You haven't set a home town yet. Use 'Set Home Town' to set one.")
                            .await?;
                    }
                }
                "back_to_main" => {
                    let keyboard = make_main_menu_keyboard();
                    bot.send_message(chat_id, "Welcome! Please choose an option:")
                        .reply_markup(keyboard)
                        .await?;
                }
                "noop" => {
                    // This button does nothing, used as a separator
                }
                "alerts" => {
                    bot.send_message(chat_id, "You pressed 'Alerts'! (Not implemented yet)")
                        .await?;
                }
                _ => {
                    // Check if it's an interested town button (format: "town_<town_name>")
                    if data.starts_with("town_") {
                        let town_name = &data[5..]; // Remove "town_" prefix
                        
                        // Send "typing" action while fetching weather
                        bot.send_chat_action(chat_id, teloxide::types::ChatAction::Typing).await?;
                        
                        match weather_api::get_current_weather(town_name).await {
                            Ok(weather) => {
                                let weather_message = weather_api::format_current_weather(&weather);
                                bot.send_message(chat_id, weather_message)
                                    .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                                    .await?;
                            }
                            Err(e) => {
                                bot.send_message(chat_id, format!("Sorry, I couldn't get the weather for '{}'. Error: {}", town_name, e))
                                    .await?;
                            }
                        }
                    } else if data.starts_with("remove_town_") {
                        let town_name = &data[12..]; // Remove "remove_town_" prefix
                        
                        update_user_data(&state, chat_id, |user_data| {
                            user_data.interested_towns.retain(|town| town != town_name);
                        });
                        
                        bot.send_message(chat_id, format!("Removed '{}' from your interested towns", town_name))
                            .await?;
                        
                        let keyboard = make_my_towns_keyboard(&state, chat_id);
                        bot.send_message(chat_id, "Manage your towns:")
                            .reply_markup(keyboard)
                            .await?;
                    } else {
                        bot.send_message(chat_id, "Unknown button.")
                            .await?;
                    }
                }
            }
        }
    }

    Ok(())
}

pub async fn message_handler(bot: Bot, msg: Message, state: SharedState) -> HandlerResult {
    if let Some(text) = msg.text() {
        let chat_id = msg.chat.id;
        let user_data = get_user_data(&state, chat_id);
        
        // Check if user is waiting for home town input
        if user_data.waiting_for_home_town {
            update_user_data(&state, chat_id, |user_data| {
                user_data.waiting_for_home_town = false;
                user_data.home_town = Some(text.to_string());
            });
            
            bot.send_message(chat_id, format!("Home town set to: {}", text))
                .await?;
            
            let keyboard = make_my_towns_keyboard(&state, chat_id);
            bot.send_message(chat_id, "Manage your towns:")
                .reply_markup(keyboard)
                .await?;
        }
        // Check if user is waiting for interested town input
        else if user_data.waiting_for_interested_town {
            update_user_data(&state, chat_id, |user_data| {
                user_data.waiting_for_interested_town = false;
                if !user_data.interested_towns.contains(&text.to_string()) {
                    user_data.interested_towns.push(text.to_string());
                }
            });
            
            bot.send_message(chat_id, format!("Added '{}' to your interested towns", text))
                .await?;
            
            let keyboard = make_my_towns_keyboard(&state, chat_id);
            bot.send_message(chat_id, "Manage your towns:")
                .reply_markup(keyboard)
                .await?;
        }
        // Check if user is waiting for city input (existing weather functionality)
        else if user_data.waiting_for_city {
            // Reset the waiting state
            update_user_data(&state, chat_id, |user_data| {
                user_data.waiting_for_city = false;
            });
            
            // Send "typing" action while fetching weather
            bot.send_chat_action(chat_id, teloxide::types::ChatAction::Typing).await?;
            
            // Fetch current weather
            match weather_api::get_current_weather(text).await {
                Ok(weather) => {
                    let weather_message = weather_api::format_current_weather(&weather);
                    bot.send_message(chat_id, weather_message)
                        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                        .await?;
                }
                Err(e) => {
                    bot.send_message(chat_id, format!("Sorry, I couldn't get the weather for '{}'. Please check the city name and try again.\n\nError: {}", text, e))
                        .await?;
                }
            }
            
            // Send "typing" action while fetching forecast
            bot.send_chat_action(chat_id, teloxide::types::ChatAction::Typing).await?;
            
            // Fetch 7-day forecast
            match weather_api::get_forecast(text, 7).await {
                Ok(forecast) => {
                    let forecast_message = weather_api::format_forecast(&forecast);
                    bot.send_message(chat_id, forecast_message)
                        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                        .await?;
                }
                Err(e) => {
                    bot.send_message(chat_id, format!("Sorry, I couldn't get the forecast for '{}'. Error: {}", text, e))
                        .await?;
                }
            }
        }
    }
    
    Ok(())
}

pub fn make_main_menu_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard = vec![];

    keyboard.push(vec![InlineKeyboardButton::callback(
        "Get Weather",
        "get_weather",
    )]);
    keyboard.push(vec![InlineKeyboardButton::callback("My Towns", "my_towns")]);
    keyboard.push(vec![InlineKeyboardButton::callback("Alerts", "alerts")]);

    InlineKeyboardMarkup::new(keyboard)
}

pub fn make_my_towns_keyboard(state: &SharedState, chat_id: ChatId) -> InlineKeyboardMarkup {
    let mut keyboard = vec![];
    let user_data = get_user_data(state, chat_id);
    
    // Home town section
    if let Some(home_town) = &user_data.home_town {
        keyboard.push(vec![InlineKeyboardButton::callback(
            &format!("🏠 Home: {} (View Weather)", home_town),
            "view_home_weather",
        )]);
        keyboard.push(vec![InlineKeyboardButton::callback(
            "Change Home Town",
            "set_home_town",
        )]);
    } else {
        keyboard.push(vec![InlineKeyboardButton::callback(
            "Set Home Town",
            "set_home_town",
        )]);
    }
    
    // Interested towns section
    if !user_data.interested_towns.is_empty() {
        keyboard.push(vec![InlineKeyboardButton::callback(
            "--- Interested Towns ---",
            "noop", // This button does nothing, just a separator
        )]);
        
        for town in &user_data.interested_towns {
            keyboard.push(vec![InlineKeyboardButton::callback(
                &format!("🌍 {}", town),
                &format!("town_{}", town),
            )]);
        }
    }
    
    // Action buttons
    keyboard.push(vec![InlineKeyboardButton::callback(
        "Add Interested Town",
        "add_interested_town",
    )]);
    
    if !user_data.interested_towns.is_empty() {
        keyboard.push(vec![InlineKeyboardButton::callback(
            "Remove Interested Town",
            "remove_interested_town",
        )]);
    }
    
    keyboard.push(vec![InlineKeyboardButton::callback(
        "← Back to Main Menu",
        "back_to_main",
    )]);

    InlineKeyboardMarkup::new(keyboard)
}

pub fn make_remove_towns_keyboard(state: &SharedState, chat_id: ChatId) -> InlineKeyboardMarkup {
    let mut keyboard = vec![];
    let user_data = get_user_data(state, chat_id);
    
    if !user_data.interested_towns.is_empty() {
        keyboard.push(vec![InlineKeyboardButton::callback(
            "--- Interested Towns ---",
            "noop", // This button does nothing, just a separator
        )]);
        
        for town in &user_data.interested_towns {
            keyboard.push(vec![InlineKeyboardButton::callback(
                &format!("🌍 {}", town),
                &format!("remove_town_{}", town),
            )]);
        }
    }
    
    keyboard.push(vec![InlineKeyboardButton::callback(
        "← Back to Main Menu",
        "back_to_main",
    )]);

    InlineKeyboardMarkup::new(keyboard)
} 