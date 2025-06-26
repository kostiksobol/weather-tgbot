use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::BotCommands,
};
use crate::{
    weather_api, 
    state::{SharedState, get_user_data, update_user_data, AlertType}, 
    alerts::{create_standard_alert, create_temperature_alert, create_wind_alert, create_humidity_alert}
};

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    #[command(description = "Display this text.")]
    Help,
    #[command(description = "Start the bot.")]
    Start,
}

pub async fn answer(bot: Bot, msg: Message, cmd: Command, state: SharedState) -> HandlerResult {
    let chat_id = msg.chat.id;
    
    match cmd {
        Command::Help => {
            bot.send_message(chat_id, Command::descriptions().to_string())
                .await?
        }
        Command::Start => {
            // Reset user state when starting - полная очистка данных пользователя
            update_user_data(&state, chat_id, |user_data| {
                user_data.waiting_for_city = false;
                user_data.waiting_for_home_town = false;
                user_data.waiting_for_interested_town = false;
                user_data.removing_interested_town = false;
                // Очищаем сохраненные города для чистого старта
                user_data.home_town = None;
                user_data.interested_towns.clear();
            });
            
            let keyboard = make_main_menu_keyboard(&state, chat_id);
            bot.send_message(chat_id, "Welcome! Please choose an option:")
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
                "current_weather_menu" => {
                    let keyboard = make_current_weather_keyboard(&state, chat_id);
                    bot.send_message(chat_id, "Choose current weather option:")
                        .reply_markup(keyboard)
                        .await?;
                }
                "forecast_menu" => {
                    let keyboard = make_forecast_keyboard(&state, chat_id);
                    bot.send_message(chat_id, "Choose forecast option:")
                        .reply_markup(keyboard)
                        .await?;
                }

                "get_weather_for" => {
                    // Set user state to waiting for city input
                    update_user_data(&state, chat_id, |user_data| {
                        user_data.waiting_for_city = true;
                    });
                    
                    let cancel_keyboard = make_cancel_keyboard();
                    bot.send_message(chat_id, "Please enter the name of the city you want to get current weather for:")
                        .reply_markup(cancel_keyboard)
                        .await?;
                }
                "get_forecast_for" => {
                    // Set user state to waiting for forecast city input
                    update_user_data(&state, chat_id, |user_data| {
                        user_data.waiting_for_forecast_city = true;
                    });
                    
                    let cancel_keyboard = make_cancel_keyboard();
                    bot.send_message(chat_id, "Please enter the name of the city you want to get forecast for:")
                        .reply_markup(cancel_keyboard)
                        .await?;
                }
                "get_forecast_home" => {
                    let user_data = get_user_data(&state, chat_id);
                    if let Some(home_town) = &user_data.home_town {
                        // Send "typing" action while fetching forecast
                        bot.send_chat_action(chat_id, teloxide::types::ChatAction::Typing).await?;
                        
                        match weather_api::get_forecast(home_town, 3).await {
                            Ok(forecast) => {
                                let forecast_message = weather_api::format_forecast(&forecast);
                                bot.send_message(chat_id, forecast_message)
                                    .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                                    .await?;
                                
                                // Отправляем главное меню для удобства
                                let keyboard = make_main_menu_keyboard(&state, chat_id);
                                bot.send_message(chat_id, "Choose another option:")
                                    .reply_markup(keyboard)
                                    .await?;
                            }
                            Err(e) => {
                                bot.send_message(chat_id, format!("Sorry, I couldn't get the forecast for your home town '{}'. Error: {}", home_town, e))
                                    .await?;
                                
                                // Отправляем главное меню даже при ошибке
                                let keyboard = make_main_menu_keyboard(&state, chat_id);
                                bot.send_message(chat_id, "Choose another option:")
                                    .reply_markup(keyboard)
                                    .await?;
                            }
                        }
                    } else {
                        // Предлагаем сразу ввести home town
                        update_user_data(&state, chat_id, |user_data| {
                            user_data.waiting_for_home_town = true;
                        });
                        
                        let cancel_keyboard = make_cancel_keyboard();
                        bot.send_message(chat_id, "You haven't set a home town yet. Please enter your home town name:")
                            .reply_markup(cancel_keyboard)
                            .await?;
                    }
                }
                "get_weather_home" => {
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
                                
                                // Отправляем главное меню для удобства
                                let keyboard = make_main_menu_keyboard(&state, chat_id);
                                bot.send_message(chat_id, "Choose another option:")
                                    .reply_markup(keyboard)
                                    .await?;
                            }
                            Err(e) => {
                                bot.send_message(chat_id, format!("Sorry, I couldn't get the weather for your home town '{}'. Error: {}", home_town, e))
                                    .await?;
                                
                                // Отправляем главное меню даже при ошибке
                                let keyboard = make_main_menu_keyboard(&state, chat_id);
                                bot.send_message(chat_id, "Choose another option:")
                                    .reply_markup(keyboard)
                                    .await?;
                            }
                        }
                    } else {
                        // Предлагаем сразу ввести home town
                        update_user_data(&state, chat_id, |user_data| {
                            user_data.waiting_for_home_town = true;
                        });
                        
                        let cancel_keyboard = make_cancel_keyboard();
                        bot.send_message(chat_id, "You haven't set a home town yet. Please enter your home town name:")
                            .reply_markup(cancel_keyboard)
                            .await?;
                    }
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
                    
                    let cancel_keyboard = make_cancel_keyboard();
                    bot.send_message(chat_id, "Please enter the name of your home town:")
                        .reply_markup(cancel_keyboard)
                        .await?;
                }
                "add_interested_town" => {
                    update_user_data(&state, chat_id, |user_data| {
                        user_data.waiting_for_interested_town = true;
                    });
                    
                    let cancel_keyboard = make_cancel_keyboard();
                    bot.send_message(chat_id, "Please enter the name of the town you're interested in:")
                        .reply_markup(cancel_keyboard)
                        .await?;
                }
                "remove_interested_town" => {
                    let user_data = get_user_data(&state, chat_id);
                    if user_data.interested_towns.is_empty() {
                        bot.send_message(chat_id, "You don't have any interested towns to remove.")
                            .await?;
                        
                        // Отправляем главное меню
                        let keyboard = make_main_menu_keyboard(&state, chat_id);
                        bot.send_message(chat_id, "Choose another option:")
                            .reply_markup(keyboard)
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
                                
                                // Отправляем главное меню
                                let keyboard = make_main_menu_keyboard(&state, chat_id);
                                bot.send_message(chat_id, "Choose another option:")
                                    .reply_markup(keyboard)
                                    .await?;
                            }
                            Err(e) => {
                                bot.send_message(chat_id, format!("Sorry, I couldn't get the weather for your home town '{}'. Error: {}", home_town, e))
                                    .await?;
                                
                                // Отправляем главное меню даже при ошибке
                                let keyboard = make_main_menu_keyboard(&state, chat_id);
                                bot.send_message(chat_id, "Choose another option:")
                                    .reply_markup(keyboard)
                                    .await?;
                            }
                        }
                    } else {
                        bot.send_message(chat_id, "You haven't set a home town yet. Use 'Set Home Town' to set one.")
                            .await?;
                        
                        // Отправляем главное меню
                        let keyboard = make_main_menu_keyboard(&state, chat_id);
                        bot.send_message(chat_id, "Choose another option:")
                            .reply_markup(keyboard)
                            .await?;
                    }
                }
                "back_to_main" => {
                    let keyboard = make_main_menu_keyboard(&state, chat_id);
                    bot.send_message(chat_id, "Welcome! Please choose an option:")
                        .reply_markup(keyboard)
                        .await?;
                }
                "back_to_interested_towns" => {
                    let keyboard = make_my_towns_keyboard(&state, chat_id);
                    bot.send_message(chat_id, "Manage your towns:")
                        .reply_markup(keyboard)
                        .await?;
                }
                "cancel" => {
                    // Reset all waiting states
                    update_user_data(&state, chat_id, |user_data| {
                        user_data.waiting_for_city = false;
                        user_data.waiting_for_forecast_city = false;
                        user_data.waiting_for_home_town = false;
                        user_data.waiting_for_interested_town = false;
                        user_data.removing_interested_town = false;
                        user_data.waiting_for_alert_city = false;
                        user_data.waiting_for_alert_temperature_min = false;
                        user_data.waiting_for_alert_temperature_max = false;
                        user_data.waiting_for_alert_wind_speed = false;
                        user_data.waiting_for_alert_humidity_min = false;
                        user_data.waiting_for_alert_humidity_max = false;
                        user_data.pending_alert_city = None;
                        user_data.pending_alert_type = None;
                    });
                    
                    let keyboard = make_main_menu_keyboard(&state, chat_id);
                    bot.send_message(chat_id, "Operation cancelled. Choose another option:")
                        .reply_markup(keyboard)
                        .await?;
                }
                "noop" => {
                    // This button does nothing, used as a separator
                }
                "alerts_menu" => {
                    let keyboard = make_alerts_menu_keyboard(&state, chat_id);
                    bot.send_message(chat_id, "🚨 Weather Alerts Management\n\nChoose an option:")
                        .reply_markup(keyboard)
                        .await?;
                }
                "add_alert" => {
                    let keyboard = make_add_alert_keyboard();
                    bot.send_message(chat_id, "Choose alert type:")
                        .reply_markup(keyboard)
                        .await?;
                }

                "remove_alert" => {
                    let user_data = get_user_data(&state, chat_id);
                    if user_data.weather_alerts.is_empty() {
                        bot.send_message(chat_id, "You don't have any alerts to remove.")
                            .await?;
                        
                        let keyboard = make_alerts_menu_keyboard(&state, chat_id);
                        bot.send_message(chat_id, "Choose another option:")
                            .reply_markup(keyboard)
                            .await?;
                    } else {
                        let keyboard = make_remove_alerts_keyboard(&state, chat_id);
                        bot.send_message(chat_id, "Select an alert to remove:")
                            .reply_markup(keyboard)
                            .await?;
                    }
                }
                "add_standard_alert" => {
                    update_user_data(&state, chat_id, |user_data| {
                        user_data.waiting_for_alert_city = true;
                        user_data.pending_alert_type = Some(AlertType::StandardWeatherAlert);
                    });
                    
                    let cancel_keyboard = make_cancel_keyboard();
                    bot.send_message(chat_id, "Enter the city name for standard weather alerts:")
                        .reply_markup(cancel_keyboard)
                        .await?;
                }
                "add_temperature_alert" => {
                    update_user_data(&state, chat_id, |user_data| {
                        user_data.waiting_for_alert_city = true;
                        user_data.pending_alert_type = Some(AlertType::TemperatureThreshold { min: None, max: None });
                    });
                    
                    let cancel_keyboard = make_cancel_keyboard();
                    bot.send_message(chat_id, "Enter the city name for temperature alerts:")
                        .reply_markup(cancel_keyboard)
                        .await?;
                }
                "add_wind_alert" => {
                    update_user_data(&state, chat_id, |user_data| {
                        user_data.waiting_for_alert_city = true;
                        user_data.pending_alert_type = Some(AlertType::WindSpeed { max: 0.0 });
                    });
                    
                    let cancel_keyboard = make_cancel_keyboard();
                    bot.send_message(chat_id, "Enter the city name for wind speed alerts:")
                        .reply_markup(cancel_keyboard)
                        .await?;
                }
                "add_humidity_alert" => {
                    update_user_data(&state, chat_id, |user_data| {
                        user_data.waiting_for_alert_city = true;
                        user_data.pending_alert_type = Some(AlertType::Humidity { min: None, max: None });
                    });
                    
                    let cancel_keyboard = make_cancel_keyboard();
                    bot.send_message(chat_id, "Enter the city name for humidity alerts:")
                        .reply_markup(cancel_keyboard)
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
                                
                                // Отправляем главное меню
                                let keyboard = make_main_menu_keyboard(&state, chat_id);
                                bot.send_message(chat_id, "Choose another option:")
                                    .reply_markup(keyboard)
                                    .await?;
                            }
                            Err(e) => {
                                bot.send_message(chat_id, format!("Sorry, I couldn't get the weather for '{}'. Error: {}", town_name, e))
                                    .await?;
                                
                                // Отправляем главное меню даже при ошибке
                                let keyboard = make_main_menu_keyboard(&state, chat_id);
                                bot.send_message(chat_id, "Choose another option:")
                                    .reply_markup(keyboard)
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
                        
                        // Возвращаемся в interested towns меню
                        let keyboard = make_my_towns_keyboard(&state, chat_id);
                        bot.send_message(chat_id, "Manage your towns:")
                            .reply_markup(keyboard)
                            .await?;
                    } else if data.starts_with("remove_alert_") {
                        let alert_id = &data[13..]; // Remove "remove_alert_" prefix
                        
                        update_user_data(&state, chat_id, |user_data| {
                            user_data.weather_alerts.retain(|alert| alert.id != alert_id);
                        });
                        
                        bot.send_message(chat_id, "Alert removed successfully!")
                            .await?;
                        
                        // Возвращаемся в alerts меню
                        let keyboard = make_alerts_menu_keyboard(&state, chat_id);
                        bot.send_message(chat_id, "Weather Alerts Management:")
                            .reply_markup(keyboard)
                            .await?;
                    } else if data.starts_with("check_alert_") {
                        let alert_id = &data[12..]; // Remove "check_alert_" prefix
                        
                        let user_data = get_user_data(&state, chat_id);
                        if let Some(alert) = user_data.weather_alerts.iter().find(|a| a.id == alert_id) {
                            // Send "typing" action while checking alert
                            bot.send_chat_action(chat_id, teloxide::types::ChatAction::Typing).await?;
                            
                            match crate::alerts::AlertChecker::check_alert(alert).await {
                                Ok(is_triggered) => {
                                    match weather_api::get_current_weather(&alert.city).await {
                                        Ok(weather) => {
                                            let status_emoji = if is_triggered { "🚨" } else { "✅" };
                                            let status_text = if is_triggered { "ALERT TRIGGERED!" } else { "All Good" };
                                            
                                                                        let alert_type_str = match &alert.alert_type {
                                AlertType::StandardWeatherAlert => "🚨 Standard Weather Alert".to_string(),
                                AlertType::TemperatureThreshold { min, max } => {
                                    let range = match (min, max) {
                                        (Some(min_val), Some(max_val)) => format!("{}°C - {}°C", min_val, max_val),
                                        (Some(min_val), None) => format!("min {}°C", min_val),
                                        (None, Some(max_val)) => format!("max {}°C", max_val),
                                        (None, None) => "Temperature".to_string(),
                                    };
                                    format!("🌡️ Temperature Alert ({})", range)
                                },
                                AlertType::WindSpeed { max } => format!("💨 Wind Speed Alert (max {} km/h)", max),
                                AlertType::Humidity { min, max } => {
                                    let range = match (min, max) {
                                        (Some(min_val), Some(max_val)) => format!("{}% - {}%", min_val, max_val),
                                        (Some(min_val), None) => format!("min {}%", min_val),
                                        (None, Some(max_val)) => format!("max {}%", max_val),
                                        (None, None) => "Humidity".to_string(),
                                    };
                                    format!("💧 Humidity Alert ({})", range)
                                },
                            };
                                            
                                                                        let message = format!(
                                "{} <b>{}</b>\n\n{}\n\n📍 <b>City:</b> {}\n📝 <b>Description:</b> {}\n\n<b>Current Weather:</b>\n🌡️ Temperature: {}°C\n☁️ Condition: {}\n💨 Wind: {} km/h\n💧 Humidity: {}%\n\n⏰ Created: {}\n{}",
                                status_emoji,
                                status_text,
                                alert_type_str,
                                weather.location.name,
                                alert.description,
                                weather.current.temperature,
                                weather.current.condition.text,
                                weather.current.wind_speed,
                                weather.current.humidity,
                                alert.created_at.format("%Y-%m-%d %H:%M"),
                                if let Some(last_triggered) = alert.last_triggered {
                                    format!("🔔 Last triggered: {}", last_triggered.format("%Y-%m-%d %H:%M"))
                                } else {
                                    "🔔 Never triggered".to_string()
                                }
                            );
                                            
                                                                        bot.send_message(chat_id, message)
                                .parse_mode(teloxide::types::ParseMode::Html)
                                .await?;
                                        }
                                        Err(e) => {
                                            bot.send_message(chat_id, format!("❌ Failed to get weather data for {}: {}", alert.city, e))
                                                .await?;
                                        }
                                    }
                                }
                                Err(e) => {
                                    bot.send_message(chat_id, format!("❌ Error checking alert: {}", e))
                                        .await?;
                                }
                            }
                        } else {
                            bot.send_message(chat_id, "❌ Alert not found.")
                                .await?;
                        }
                        
                        // Возвращаемся в alerts меню
                        let keyboard = make_alerts_menu_keyboard(&state, chat_id);
                        bot.send_message(chat_id, "Weather Alerts Management:")
                            .reply_markup(keyboard)
                            .await?;
                    } else {
                    bot.send_message(chat_id, "Unknown button.")
                        .await?;
                        
                        // Отправляем главное меню
                        let keyboard = make_main_menu_keyboard(&state, chat_id);
                        bot.send_message(chat_id, "Choose another option:")
                            .reply_markup(keyboard)
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
            
            // Отправляем главное меню для удобства
            let keyboard = make_main_menu_keyboard(&state, chat_id);
            bot.send_message(chat_id, "Choose another option:")
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
            
            // Возвращаемся в interested towns меню
            let keyboard = make_my_towns_keyboard(&state, chat_id);
            bot.send_message(chat_id, "Manage your towns:")
                .reply_markup(keyboard)
                .await?;
        }
                // Check if user is waiting for city input (current weather only)
        else if user_data.waiting_for_city {
            // Reset the waiting state
            update_user_data(&state, chat_id, |user_data| {
                user_data.waiting_for_city = false;
            });
            
            // Send "typing" action while fetching weather
            bot.send_chat_action(chat_id, teloxide::types::ChatAction::Typing).await?;
            
            // Fetch ONLY current weather
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
            
            // Return to main menu after weather
            let keyboard = make_main_menu_keyboard(&state, chat_id);
            bot.send_message(chat_id, "Choose another option:")
                .reply_markup(keyboard)
                .await?;
        }
        // Check if user is waiting for forecast city input
        else if user_data.waiting_for_forecast_city {
            // Reset the waiting state
            update_user_data(&state, chat_id, |user_data| {
                user_data.waiting_for_forecast_city = false;
            });
            
            // Send "typing" action while fetching forecast
            bot.send_chat_action(chat_id, teloxide::types::ChatAction::Typing).await?;
            
            // Fetch 3-day forecast  
            match weather_api::get_forecast(text, 3).await {
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
            
            // Return to main menu after forecast
            let keyboard = make_main_menu_keyboard(&state, chat_id);
            bot.send_message(chat_id, "Choose another option:")
                .reply_markup(keyboard)
                .await?;
        }
        // Check if user is waiting for alert city input
        else if user_data.waiting_for_alert_city {
            update_user_data(&state, chat_id, |user_data| {
                user_data.waiting_for_alert_city = false;
                user_data.pending_alert_city = Some(text.to_string());
            });
            
            let user_data = get_user_data(&state, chat_id);
            match &user_data.pending_alert_type {
                Some(AlertType::StandardWeatherAlert) => {
                    let alert = create_standard_alert(text.to_string());
                    update_user_data(&state, chat_id, |user_data| {
                        user_data.weather_alerts.push(alert);
                        user_data.pending_alert_city = None;
                        user_data.pending_alert_type = None;
                    });
                    
                    bot.send_message(chat_id, format!("✅ Standard weather alert created for '{}'!", text))
                        .await?;
                    
                    let keyboard = make_alerts_menu_keyboard(&state, chat_id);
                    bot.send_message(chat_id, "Weather Alerts Management:")
                        .reply_markup(keyboard)
                        .await?;
                }
                Some(AlertType::TemperatureThreshold { .. }) => {
                    update_user_data(&state, chat_id, |user_data| {
                        user_data.waiting_for_alert_temperature_min = true;
                    });
                    
                    let cancel_keyboard = make_cancel_keyboard();
                    bot.send_message(chat_id, "Enter minimum temperature threshold (°C) or type 'skip' to skip:")
                        .reply_markup(cancel_keyboard)
                        .await?;
                }
                Some(AlertType::WindSpeed { .. }) => {
                    update_user_data(&state, chat_id, |user_data| {
                        user_data.waiting_for_alert_wind_speed = true;
                    });
                    
                    let cancel_keyboard = make_cancel_keyboard();
                    bot.send_message(chat_id, "Enter maximum wind speed threshold (km/h):")
                        .reply_markup(cancel_keyboard)
                        .await?;
                }
                Some(AlertType::Humidity { .. }) => {
                    update_user_data(&state, chat_id, |user_data| {
                        user_data.waiting_for_alert_humidity_min = true;
                    });
                    
                    let cancel_keyboard = make_cancel_keyboard();
                    bot.send_message(chat_id, "Enter minimum humidity threshold (%) or type 'skip' to skip:")
                        .reply_markup(cancel_keyboard)
                        .await?;
                }
                _ => {
                    bot.send_message(chat_id, "Error: Unknown alert type. Please try again.")
                        .await?;
                    
                    let keyboard = make_alerts_menu_keyboard(&state, chat_id);
                    bot.send_message(chat_id, "Weather Alerts Management:")
                        .reply_markup(keyboard)
                        .await?;
                }
            }
        }
        // Handle temperature alert parameters
        else if user_data.waiting_for_alert_temperature_min {
            let min_temp = if text.to_lowercase() == "skip" {
                None
            } else {
                match text.parse::<f32>() {
                    Ok(temp) => Some(temp),
                    Err(_) => {
                        bot.send_message(chat_id, "Invalid temperature value. Please enter a valid number or 'skip':")
                            .await?;
                        return Ok(());
                    }
                }
            };
            
            update_user_data(&state, chat_id, |user_data| {
                user_data.waiting_for_alert_temperature_min = false;
                user_data.waiting_for_alert_temperature_max = true;
                if let Some(AlertType::TemperatureThreshold { min, .. }) = &mut user_data.pending_alert_type {
                    *min = min_temp;
                }
            });
            
            let cancel_keyboard = make_cancel_keyboard();
            bot.send_message(chat_id, "Enter maximum temperature threshold (°C) or type 'skip' to skip:")
                .reply_markup(cancel_keyboard)
                .await?;
        }
        else if user_data.waiting_for_alert_temperature_max {
            let max_temp = if text.to_lowercase() == "skip" {
                None
            } else {
                match text.parse::<f32>() {
                    Ok(temp) => Some(temp),
                    Err(_) => {
                        bot.send_message(chat_id, "Invalid temperature value. Please enter a valid number or 'skip':")
                            .await?;
                        return Ok(());
                    }
                }
            };
            
            let user_data_clone = get_user_data(&state, chat_id);
            if let (Some(city), Some(AlertType::TemperatureThreshold { min, .. })) = 
                (&user_data_clone.pending_alert_city, &user_data_clone.pending_alert_type) {
                let alert = create_temperature_alert(city.clone(), *min, max_temp);
                update_user_data(&state, chat_id, |user_data| {
                    user_data.weather_alerts.push(alert);
                    user_data.waiting_for_alert_temperature_max = false;
                    user_data.pending_alert_city = None;
                    user_data.pending_alert_type = None;
                });
                
                bot.send_message(chat_id, format!("✅ Temperature alert created for '{}'!", city))
                    .await?;
                
                let keyboard = make_alerts_menu_keyboard(&state, chat_id);
                bot.send_message(chat_id, "Weather Alerts Management:")
                    .reply_markup(keyboard)
                    .await?;
            }
        }
        // Handle wind speed alert
        else if user_data.waiting_for_alert_wind_speed {
            match text.parse::<f32>() {
                Ok(wind_speed) => {
                    let user_data_clone = get_user_data(&state, chat_id);
                    if let Some(city) = &user_data_clone.pending_alert_city {
                        let alert = create_wind_alert(city.clone(), wind_speed);
                        update_user_data(&state, chat_id, |user_data| {
                            user_data.weather_alerts.push(alert);
                            user_data.waiting_for_alert_wind_speed = false;
                            user_data.pending_alert_city = None;
                            user_data.pending_alert_type = None;
                        });
                        
                        bot.send_message(chat_id, format!("✅ Wind speed alert created for '{}'!", city))
                            .await?;
                        
                        let keyboard = make_alerts_menu_keyboard(&state, chat_id);
                        bot.send_message(chat_id, "Weather Alerts Management:")
                            .reply_markup(keyboard)
                            .await?;
                    }
                }
                Err(_) => {
                    bot.send_message(chat_id, "Invalid wind speed value. Please enter a valid number:")
                        .await?;
                }
            }
        }
        // Handle humidity alert parameters
        else if user_data.waiting_for_alert_humidity_min {
            let min_humidity = if text.to_lowercase() == "skip" {
                None
            } else {
                match text.parse::<u32>() {
                    Ok(humidity) => Some(humidity),
                    Err(_) => {
                        bot.send_message(chat_id, "Invalid humidity value. Please enter a valid number (0-100) or 'skip':")
                            .await?;
                        return Ok(());
                    }
                }
            };
            
            update_user_data(&state, chat_id, |user_data| {
                user_data.waiting_for_alert_humidity_min = false;
                user_data.waiting_for_alert_humidity_max = true;
                if let Some(AlertType::Humidity { min, .. }) = &mut user_data.pending_alert_type {
                    *min = min_humidity;
                }
            });
            
            let cancel_keyboard = make_cancel_keyboard();
            bot.send_message(chat_id, "Enter maximum humidity threshold (%) or type 'skip' to skip:")
                .reply_markup(cancel_keyboard)
                .await?;
        }
        else if user_data.waiting_for_alert_humidity_max {
            let max_humidity = if text.to_lowercase() == "skip" {
                None
            } else {
                match text.parse::<u32>() {
                    Ok(humidity) => Some(humidity),
                    Err(_) => {
                        bot.send_message(chat_id, "Invalid humidity value. Please enter a valid number (0-100) or 'skip':")
                            .await?;
                        return Ok(());
                    }
                }
            };
            
            let user_data_clone = get_user_data(&state, chat_id);
            if let (Some(city), Some(AlertType::Humidity { min, .. })) = 
                (&user_data_clone.pending_alert_city, &user_data_clone.pending_alert_type) {
                let alert = create_humidity_alert(city.clone(), *min, max_humidity);
                update_user_data(&state, chat_id, |user_data| {
                    user_data.weather_alerts.push(alert);
                    user_data.waiting_for_alert_humidity_max = false;
                    user_data.pending_alert_city = None;
                    user_data.pending_alert_type = None;
                });
                
                bot.send_message(chat_id, format!("✅ Humidity alert created for '{}'!", city))
                    .await?;
                
                let keyboard = make_alerts_menu_keyboard(&state, chat_id);
                bot.send_message(chat_id, "Weather Alerts Management:")
                    .reply_markup(keyboard)
                    .await?;
            }
        }
    }
    
    Ok(())
}

pub fn make_main_menu_keyboard(_state: &SharedState, _chat_id: ChatId) -> InlineKeyboardMarkup {
    let mut keyboard = vec![];

    // Красиво организованное главное меню
    keyboard.push(vec![InlineKeyboardButton::callback(
        "Current weather",
        "current_weather_menu",
    )]);
    
    keyboard.push(vec![InlineKeyboardButton::callback(
        "Forecast", 
        "forecast_menu"
    )]);
    
    keyboard.push(vec![InlineKeyboardButton::callback("Interested towns", "my_towns")]);
    
    keyboard.push(vec![InlineKeyboardButton::callback("🚨 Weather Alerts", "alerts_menu")]);

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
            "🔄 Change Home Town",
            "set_home_town",
        )]);
    } else {
        keyboard.push(vec![InlineKeyboardButton::callback(
            "🏠 Set Home Town",
            "set_home_town",
        )]);
    }
    
    // Interested towns section
    if !user_data.interested_towns.is_empty() {
        keyboard.push(vec![InlineKeyboardButton::callback(
            "--- 🌍 Interested Towns ---",
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
        "➕ Add Interested Town",
        "add_interested_town",
    )]);
    
    if !user_data.interested_towns.is_empty() {
        keyboard.push(vec![InlineKeyboardButton::callback(
            "🗑️ Remove Interested Town",
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
            "--- 🗑️ Select Town to Remove ---",
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
        "← Back to Interested Towns",
        "back_to_interested_towns",
    )]);

    InlineKeyboardMarkup::new(keyboard)
}

pub fn make_cancel_keyboard() -> InlineKeyboardMarkup {
    let keyboard = vec![
        vec![InlineKeyboardButton::callback("Cancel", "cancel")],
    ];

    InlineKeyboardMarkup::new(keyboard)
}

pub fn make_current_weather_keyboard(_state: &SharedState, _chat_id: ChatId) -> InlineKeyboardMarkup {
    let mut keyboard = vec![];

    keyboard.push(vec![InlineKeyboardButton::callback(
        "For any city",
        "get_weather_for",
    )]);
    
    keyboard.push(vec![InlineKeyboardButton::callback(
        "For home",
        "get_weather_home",
    )]);
    
    keyboard.push(vec![InlineKeyboardButton::callback(
        "← Back to Main Menu",
        "back_to_main",
    )]);

    InlineKeyboardMarkup::new(keyboard)
}

pub fn make_forecast_keyboard(_state: &SharedState, _chat_id: ChatId) -> InlineKeyboardMarkup {
    let mut keyboard = vec![];

    keyboard.push(vec![InlineKeyboardButton::callback(
        "For any city",
        "get_forecast_for",
    )]);
    
    keyboard.push(vec![InlineKeyboardButton::callback(
        "For home",
        "get_forecast_home",
    )]);
    
    keyboard.push(vec![InlineKeyboardButton::callback(
        "← Back to Main Menu",
        "back_to_main",
    )]);

    InlineKeyboardMarkup::new(keyboard)
}

pub fn make_alerts_menu_keyboard(state: &SharedState, chat_id: ChatId) -> InlineKeyboardMarkup {
    let mut keyboard = vec![];
    let user_data = get_user_data(state, chat_id);
    
    // Показываем список алертов, если они есть
    if !user_data.weather_alerts.is_empty() {
        keyboard.push(vec![InlineKeyboardButton::callback(
            "--- 🚨 Your Weather Alerts ---",
            "noop",
        )]);
        
        for alert in &user_data.weather_alerts {
            let alert_type_emoji = match &alert.alert_type {
                AlertType::StandardWeatherAlert => "🚨",
                AlertType::TemperatureThreshold { .. } => "🌡️",
                AlertType::WindSpeed { .. } => "💨",
                AlertType::Humidity { .. } => "💧",
            };
            
            let status = if alert.is_active { "✅" } else { "❌" };
            
            keyboard.push(vec![InlineKeyboardButton::callback(
                &format!("{} {} - {} {}", alert_type_emoji, alert.city, 
                        match &alert.alert_type {
                            AlertType::StandardWeatherAlert => "Standard",
                            AlertType::TemperatureThreshold { .. } => "Temperature",
                            AlertType::WindSpeed { .. } => "Wind",
                            AlertType::Humidity { .. } => "Humidity",
                        }, status),
                &format!("check_alert_{}", alert.id),
            )]);
        }
    }
    
    // Кнопки управления
    keyboard.push(vec![InlineKeyboardButton::callback(
        "➕ Add Alert",
        "add_alert",
    )]);
    
    if !user_data.weather_alerts.is_empty() {
        keyboard.push(vec![InlineKeyboardButton::callback(
            "🗑️ Remove Alert",
            "remove_alert",
        )]);
    }
    
    keyboard.push(vec![InlineKeyboardButton::callback(
        "← Back to Main Menu",
        "back_to_main",
    )]);

    InlineKeyboardMarkup::new(keyboard)
}

pub fn make_add_alert_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard = vec![];
    
    keyboard.push(vec![InlineKeyboardButton::callback(
        "🚨 Standard Weather Alert",
        "add_standard_alert",
    )]);
    
    keyboard.push(vec![InlineKeyboardButton::callback(
        "🌡️ Temperature Alert",
        "add_temperature_alert",
    )]);
    
    keyboard.push(vec![InlineKeyboardButton::callback(
        "💨 Wind Speed Alert",
        "add_wind_alert",
    )]);
    
    keyboard.push(vec![InlineKeyboardButton::callback(
        "💧 Humidity Alert",
        "add_humidity_alert",
    )]);
    
    keyboard.push(vec![InlineKeyboardButton::callback(
        "← Back to Alerts Menu",
        "alerts_menu",
    )]);

    InlineKeyboardMarkup::new(keyboard)
}

pub fn make_remove_alerts_keyboard(state: &SharedState, chat_id: ChatId) -> InlineKeyboardMarkup {
    let mut keyboard = vec![];
    let user_data = get_user_data(state, chat_id);
    
    if !user_data.weather_alerts.is_empty() {
        keyboard.push(vec![InlineKeyboardButton::callback(
            "--- 🗑️ Select Alert to Remove ---",
            "noop",
        )]);
        
        for alert in &user_data.weather_alerts {
            let alert_type_str = match &alert.alert_type {
                AlertType::StandardWeatherAlert => "🚨 Standard",
                AlertType::TemperatureThreshold { .. } => "🌡️ Temperature",
                AlertType::WindSpeed { .. } => "💨 Wind",
                AlertType::Humidity { .. } => "💧 Humidity",
            };
            
            let button_text = format!("{} - {} ({})", alert_type_str, alert.city, 
                                    if alert.is_active { "Active" } else { "Inactive" });
            
            keyboard.push(vec![InlineKeyboardButton::callback(
                &button_text,
                &format!("remove_alert_{}", alert.id),
            )]);
        }
    }
    
    keyboard.push(vec![InlineKeyboardButton::callback(
        "← Back to Alerts Menu",
        "alerts_menu",
    )]);

    InlineKeyboardMarkup::new(keyboard)
} 