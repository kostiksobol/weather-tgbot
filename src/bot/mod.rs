use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::BotCommands,
};
use crate::{
    weather_api, 
    state::{SharedState, get_user_data, update_user_data, AlertType}, 
    alerts::{create_standard_alert, create_temperature_alert, create_wind_alert, create_humidity_alert},
    i18n::{Language, t},
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
            // Check if user already has a language set
            let user_data = get_user_data(&state, chat_id);
            if user_data.language == Language::default() && user_data.home_town.is_none() {
                // First time user - show language selection
                let keyboard = make_language_selection_keyboard();
                bot.send_message(chat_id, "Please select your language / Пожалуйста, выберите ваш язык:")
                    .reply_markup(keyboard)
                    .await?
            } else {
                // Returning user - show main menu
                let lang = user_data.language;
                let keyboard = make_main_menu_keyboard(&state, chat_id);
                bot.send_message(chat_id, t("welcome", lang))
                    .reply_markup(keyboard)
                    .await?
            }
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
            let lang = get_user_data(&state, chat_id).language;
            
            match data.as_str() {
                "language_en" => {
                    update_user_data(&state, chat_id, |user_data| {
                        user_data.language = Language::English;
                    });
                    
                    bot.send_message(chat_id, t("language_changed", Language::English))
                        .await?;
                    
                    let keyboard = make_main_menu_keyboard(&state, chat_id);
                    bot.send_message(chat_id, t("welcome", Language::English))
                        .reply_markup(keyboard)
                        .await?;
                }
                "language_ru" => {
                    update_user_data(&state, chat_id, |user_data| {
                        user_data.language = Language::Russian;
                    });
                    
                    bot.send_message(chat_id, t("language_changed", Language::Russian))
                        .await?;
                    
                    let keyboard = make_main_menu_keyboard(&state, chat_id);
                    bot.send_message(chat_id, t("welcome", Language::Russian))
                        .reply_markup(keyboard)
                        .await?;
                }
                "settings_menu" => {
                    let keyboard = make_settings_keyboard(&state, chat_id);
                    bot.send_message(chat_id, t("settings_menu", lang))
                        .reply_markup(keyboard)
                        .await?;
                }
                "change_language" => {
                    let keyboard = make_language_selection_keyboard();
                    bot.send_message(chat_id, t("language_selection", lang))
                        .reply_markup(keyboard)
                        .await?;
                }
                "current_weather_menu" => {
                    let keyboard = make_current_weather_keyboard(&state, chat_id);
                    bot.send_message(chat_id, t("choose_weather_option", lang))
                        .reply_markup(keyboard)
                        .await?;
                }
                "forecast_menu" => {
                    let keyboard = make_forecast_keyboard(&state, chat_id);
                    bot.send_message(chat_id, t("choose_forecast_option", lang))
                        .reply_markup(keyboard)
                        .await?;
                }

                "get_weather_for" => {
                    // Set user state to waiting for city input
                    update_user_data(&state, chat_id, |user_data| {
                        user_data.waiting_for_city = true;
                    });
                    
                    let cancel_keyboard = make_cancel_keyboard(&state, chat_id);
                    bot.send_message(chat_id, t("enter_city_weather", lang))
                        .reply_markup(cancel_keyboard)
                        .await?;
                }
                "get_forecast_for" => {
                    // Set user state to waiting for forecast city input
                    update_user_data(&state, chat_id, |user_data| {
                        user_data.waiting_for_forecast_city = true;
                    });
                    
                    let cancel_keyboard = make_cancel_keyboard(&state, chat_id);
                    bot.send_message(chat_id, t("enter_city_forecast", lang))
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
                        
                        let cancel_keyboard = make_cancel_keyboard(&state, chat_id);
                        bot.send_message(chat_id, t("no_home_town", lang))
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
                                bot.send_message(chat_id, t("choose_another_option", lang))
                                    .reply_markup(keyboard)
                                    .await?;
                            }
                            Err(e) => {
                                bot.send_message(chat_id, format!("{} '{}'. {}: {}", 
                                    t("error_weather", lang), home_town, t("error", lang), e))
                                    .await?;
                                
                                // Отправляем главное меню даже при ошибке
                                let keyboard = make_main_menu_keyboard(&state, chat_id);
                                bot.send_message(chat_id, t("choose_another_option", lang))
                                    .reply_markup(keyboard)
                                    .await?;
                            }
                        }
                    } else {
                        bot.send_message(chat_id, t("no_home_town_use_set", lang))
                            .await?;
                        
                        // Отправляем главное меню
                        let keyboard = make_main_menu_keyboard(&state, chat_id);
                        bot.send_message(chat_id, t("choose_another_option", lang))
                            .reply_markup(keyboard)
                            .await?;
                    }
                }
                "my_towns" => {
                    let keyboard = make_my_towns_keyboard(&state, chat_id);
                    bot.send_message(chat_id, t("manage_towns", lang))
                        .reply_markup(keyboard)
                        .await?;
                }
                "set_home_town" => {
                    update_user_data(&state, chat_id, |user_data| {
                        user_data.waiting_for_home_town = true;
                    });
                    
                    let cancel_keyboard = make_cancel_keyboard(&state, chat_id);
                    bot.send_message(chat_id, t("enter_home_town", lang))
                        .reply_markup(cancel_keyboard)
                        .await?;
                }
                "add_interested_town" => {
                    update_user_data(&state, chat_id, |user_data| {
                        user_data.waiting_for_interested_town = true;
                    });
                    
                    let cancel_keyboard = make_cancel_keyboard(&state, chat_id);
                    bot.send_message(chat_id, t("enter_interested_town", lang))
                        .reply_markup(cancel_keyboard)
                        .await?;
                }
                "remove_interested_town" => {
                    let user_data = get_user_data(&state, chat_id);
                    if user_data.interested_towns.is_empty() {
                        bot.send_message(chat_id, t("no_towns_to_remove", lang))
                            .await?;
                        
                        // Отправляем главное меню
                        let keyboard = make_main_menu_keyboard(&state, chat_id);
                        bot.send_message(chat_id, t("choose_another_option", lang))
                            .reply_markup(keyboard)
                            .await?;
                    } else {
                        let keyboard = make_remove_towns_keyboard(&state, chat_id);
                        bot.send_message(chat_id, t("select_town_to_remove", lang))
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
                        bot.send_message(chat_id, t("no_home_town_use_set", lang))
                            .await?;
                        
                        // Отправляем главное меню
                        let keyboard = make_main_menu_keyboard(&state, chat_id);
                        bot.send_message(chat_id, t("choose_another_option", lang))
                            .reply_markup(keyboard)
                            .await?;
                    }
                }
                "back_to_main" => {
                    let keyboard = make_main_menu_keyboard(&state, chat_id);
                    bot.send_message(chat_id, t("welcome", lang))
                        .reply_markup(keyboard)
                        .await?;
                }
                "back_to_interested_towns" => {
                    let keyboard = make_my_towns_keyboard(&state, chat_id);
                    bot.send_message(chat_id, t("manage_towns", lang))
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
                        user_data.waiting_for_alert_hours_input = false;
                        user_data.pending_alert_city = None;
                        user_data.pending_alert_type = None;
                        user_data.pending_alert_hours = None;
                    });
                    
                    let keyboard = make_main_menu_keyboard(&state, chat_id);
                    bot.send_message(chat_id, t("operation_cancelled", lang))
                        .reply_markup(keyboard)
                        .await?;
                }
                "noop" => {
                    // This button does nothing, used as a separator
                }
                "alerts_menu" => {
                    let keyboard = make_alerts_menu_keyboard(&state, chat_id);
                    bot.send_message(chat_id, t("alerts_management", lang))
                        .reply_markup(keyboard)
                        .await?;
                }
                "add_alert" => {
                    let keyboard = make_add_alert_keyboard(&state, chat_id);
                    bot.send_message(chat_id, t("choose_alert_type", lang))
                        .reply_markup(keyboard)
                        .await?;
                }

                "remove_alert" => {
                    let user_data = get_user_data(&state, chat_id);
                    if user_data.weather_alerts.is_empty() {
                        bot.send_message(chat_id, t("no_alerts", lang))
                            .await?;
                        
                        let keyboard = make_alerts_menu_keyboard(&state, chat_id);
                        bot.send_message(chat_id, t("choose_another_option", lang))
                            .reply_markup(keyboard)
                            .await?;
                    } else {
                        let keyboard = make_remove_alerts_keyboard(&state, chat_id);
                        bot.send_message(chat_id, t("select_alert_to_remove", lang))
                            .reply_markup(keyboard)
                            .await?;
                    }
                }
                "add_standard_alert" => {
                    update_user_data(&state, chat_id, |user_data| {
                        user_data.waiting_for_alert_city = true;
                        user_data.pending_alert_type = Some(AlertType::StandardWeatherAlert);
                    });
                    
                    let cancel_keyboard = make_cancel_keyboard(&state, chat_id);
                    bot.send_message(chat_id, t("enter_city_standard_alert", lang))
                        .reply_markup(cancel_keyboard)
                        .await?;
                }
                "add_temperature_alert" => {
                    update_user_data(&state, chat_id, |user_data| {
                        user_data.waiting_for_alert_city = true;
                        user_data.pending_alert_type = Some(AlertType::TemperatureThreshold { min: None, max: None });
                    });
                    
                    let cancel_keyboard = make_cancel_keyboard(&state, chat_id);
                    bot.send_message(chat_id, t("enter_city_temperature_alert", lang))
                        .reply_markup(cancel_keyboard)
                        .await?;
                }
                "add_wind_alert" => {
                    update_user_data(&state, chat_id, |user_data| {
                        user_data.waiting_for_alert_city = true;
                        user_data.pending_alert_type = Some(AlertType::WindSpeed { max: 0.0 });
                    });
                    
                    let cancel_keyboard = make_cancel_keyboard(&state, chat_id);
                    bot.send_message(chat_id, t("enter_city_wind_alert", lang))
                        .reply_markup(cancel_keyboard)
                        .await?;
                }
                "add_humidity_alert" => {
                    update_user_data(&state, chat_id, |user_data| {
                        user_data.waiting_for_alert_city = true;
                        user_data.pending_alert_type = Some(AlertType::Humidity { min: None, max: None });
                    });
                    
                    let cancel_keyboard = make_cancel_keyboard(&state, chat_id);
                    bot.send_message(chat_id, t("enter_city_humidity_alert", lang))
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
                                bot.send_message(chat_id, t("choose_another_option", lang))
                                    .reply_markup(keyboard)
                                    .await?;
                            }
                            Err(e) => {
                                bot.send_message(chat_id, format!("{} '{}'. {}: {}", 
                                    t("error_weather", lang), town_name, t("error", lang), e))
                                    .await?;
                                
                                // Отправляем главное меню даже при ошибке
                                let keyboard = make_main_menu_keyboard(&state, chat_id);
                                bot.send_message(chat_id, t("choose_another_option", lang))
                                    .reply_markup(keyboard)
                                    .await?;
                            }
                        }
                    } else if data.starts_with("remove_town_") {
                        let town_name = &data[12..]; // Remove "remove_town_" prefix
                        
                        update_user_data(&state, chat_id, |user_data| {
                            user_data.interested_towns.retain(|town| town != town_name);
                        });
                        
                        bot.send_message(chat_id, format!("{} '{}'", t("town_removed", lang), town_name))
                            .await?;
                        
                        // Возвращаемся в interested towns меню
                        let keyboard = make_my_towns_keyboard(&state, chat_id);
                        bot.send_message(chat_id, t("manage_towns", lang))
                            .reply_markup(keyboard)
                            .await?;
                    } else if data.starts_with("remove_alert_") {
                        let alert_id = &data[13..]; // Remove "remove_alert_" prefix
                        
                        update_user_data(&state, chat_id, |user_data| {
                            user_data.weather_alerts.retain(|alert| alert.id != alert_id);
                        });
                        
                        bot.send_message(chat_id, t("alert_removed", lang))
                            .await?;
                        
                        // Возвращаемся в alerts меню
                        let keyboard = make_alerts_menu_keyboard(&state, chat_id);
                        bot.send_message(chat_id, t("alerts_management", lang))
                            .reply_markup(keyboard)
                            .await?;
                    } else if data.starts_with("check_alert_") {
                        let alert_id = &data[12..]; // Remove "check_alert_" prefix
                        
                        let user_data = get_user_data(&state, chat_id);
                        if let Some(alert) = user_data.weather_alerts.iter().find(|a| a.id == alert_id) {
                            // Send "typing" action while checking alert
                            bot.send_chat_action(chat_id, teloxide::types::ChatAction::Typing).await?;
                            
                            match crate::alerts::AlertChecker::check_current_alert(alert).await {
                                Ok(is_triggered) => {
                                    match weather_api::get_current_weather(&alert.city).await {
                                        Ok(weather) => {
                                            let status_emoji = if is_triggered { "🚨" } else { "✅" };
                                            let status_text = if is_triggered { 
                                                t("alert_triggered", lang) 
                                            } else { 
                                                t("all_good", lang) 
                                            };
                                            
                                            let alert_type_str = match &alert.alert_type {
                                                AlertType::StandardWeatherAlert => t("standard_weather_alert", lang).to_string(),
                                                AlertType::TemperatureThreshold { min, max } => {
                                                    let range = match (min, max) {
                                                        (Some(min_val), Some(max_val)) => format!("{}°C - {}°C", min_val, max_val),
                                                        (Some(min_val), None) => format!("{} {}°C", t("min", lang), min_val),
                                                        (None, Some(max_val)) => format!("{} {}°C", t("max", lang), max_val),
                                                        (None, None) => t("temperature", lang).to_string(),
                                                    };
                                                    format!("{} ({})", t("temperature_alert_desc", lang), range)
                                                },
                                                AlertType::WindSpeed { max } => {
                                                    let wind_desc = t("wind_speed_alert_desc", lang).replace("{}", &max.to_string());
                                                    wind_desc
                                                },
                                                AlertType::Humidity { min, max } => {
                                                    let range = match (min, max) {
                                                        (Some(min_val), Some(max_val)) => format!("{}% - {}%", min_val, max_val),
                                                        (Some(min_val), None) => format!("{} {}%", t("min", lang), min_val),
                                                        (None, Some(max_val)) => format!("{} {}%", t("max", lang), max_val),
                                                        (None, None) => t("humidity", lang).to_string(),
                                                    };
                                                    format!("{} ({})", t("humidity_alert_desc", lang), range)
                                                },
                                            };
                                            
                                            let message = format!(
                                                "{} <b>{}</b>\n\n{}\n\n📍 <b>{}:</b> {}\n📝 <b>{}:</b> {}\n\n<b>{}:</b>\n🌡️ {}: {}°C\n☁️ {}: {}\n💨 {}: {} {}\n💧 {}: {}%\n\n⏰ {}: {}\n{}",
                                                status_emoji,
                                                status_text,
                                                alert_type_str,
                                                t("city", lang),
                                                weather.location.name,
                                                t("description", lang),
                                                alert.description,
                                                t("current_weather", lang),
                                                t("temperature", lang),
                                                weather.current.temperature,
                                                t("condition", lang),
                                                weather.current.condition.text,
                                                t("wind", lang),
                                                weather.current.wind_speed,
                                                t("kmh", lang),
                                                t("humidity", lang),
                                                weather.current.humidity,
                                                t("created", lang),
                                                alert.created_at.format("%Y-%m-%d %H:%M"),
                                                if let Some(last_triggered) = alert.last_triggered {
                                                    format!("🔔 {}: {}", t("last_triggered", lang), last_triggered.format("%Y-%m-%d %H:%M"))
                                                } else {
                                                    format!("🔔 {}", t("never_triggered", lang))
                                                }
                                            );
                                            
                                            bot.send_message(chat_id, message)
                                                .parse_mode(teloxide::types::ParseMode::Html)
                                                .await?;
                                        }
                                        Err(e) => {
                                            let error_msg = t("failed_weather_data", lang)
                                                .replace("{}", &alert.city)
                                                .replace("{}", &e.to_string());
                                            bot.send_message(chat_id, error_msg)
                                                .await?;
                                        }
                                    }
                                }
                                Err(e) => {
                                    let error_msg = t("error_checking_alert", lang)
                                        .replace("{}", &e.to_string());
                                    bot.send_message(chat_id, error_msg)
                                        .await?;
                                }
                            }
                        } else {
                            bot.send_message(chat_id, t("alert_not_found", lang))
                                .await?;
                        }
                            
                        // Возвращаемся в alerts меню
                        let keyboard = make_alerts_menu_keyboard(&state, chat_id);
                        bot.send_message(chat_id, t("alerts_management", lang))
                            .reply_markup(keyboard)
                            .await?;

                    } else {
                        bot.send_message(chat_id, t("unknown_button", lang))
                            .await?;
                        
                        // Отправляем главное меню
                        let keyboard = make_main_menu_keyboard(&state, chat_id);
                        bot.send_message(chat_id, t("choose_another_option", lang))
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
        let lang = user_data.language;
        
        // Check if user is waiting for home town input
        if user_data.waiting_for_home_town {
            update_user_data(&state, chat_id, |user_data| {
                user_data.waiting_for_home_town = false;
                user_data.home_town = Some(text.to_string());
            });
            
            bot.send_message(chat_id, format!("{} {}", t("home_town_set", lang), text))
                .await?;
            
            // Отправляем главное меню для удобства
            let keyboard = make_main_menu_keyboard(&state, chat_id);
            bot.send_message(chat_id, t("choose_another_option", lang))
                .reply_markup(keyboard)
                .await?;
        }
        // Check if user is waiting for interested town input
        else if user_data.waiting_for_interested_town {
            let user_data_clone = get_user_data(&state, chat_id);
            if user_data_clone.interested_towns.contains(&text.to_string()) {
                bot.send_message(chat_id, t("town_already_exists", lang))
                    .await?;
            } else {
                update_user_data(&state, chat_id, |user_data| {
                    user_data.waiting_for_interested_town = false;
                    user_data.interested_towns.push(text.to_string());
                });
                
                bot.send_message(chat_id, format!("{} {}", t("town_added", lang), text))
                    .await?;
            }
            
            // Возвращаемся в interested towns меню
            let keyboard = make_my_towns_keyboard(&state, chat_id);
            bot.send_message(chat_id, t("manage_towns", lang))
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
                    let error_msg = t("weather_error_check_city", lang)
                        .replace("{}", text)
                        .replace("{}", &e.to_string());
                    bot.send_message(chat_id, error_msg)
                        .await?;
                }
            }
            
            // Return to main menu after weather
            let keyboard = make_main_menu_keyboard(&state, chat_id);
            bot.send_message(chat_id, t("choose_another_option", lang))
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
                    bot.send_message(chat_id, format!("{} '{}'. {}: {}", 
                        t("error_forecast", lang), text, t("error", lang), e))
                        .await?;
                }
            }
            
            // Return to main menu after forecast
            let keyboard = make_main_menu_keyboard(&state, chat_id);
            bot.send_message(chat_id, t("choose_another_option", lang))
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
                    update_user_data(&state, chat_id, |user_data| {
                        user_data.waiting_for_alert_hours_input = true;
                    });
                    
                    let cancel_keyboard = make_cancel_keyboard(&state, chat_id);
                    let message = t("how_many_hours", lang).replace("{}", text);
                    bot.send_message(chat_id, message)
                        .reply_markup(cancel_keyboard)
                        .await?;
                }
                Some(AlertType::TemperatureThreshold { .. }) => {
                    update_user_data(&state, chat_id, |user_data| {
                        user_data.waiting_for_alert_temperature_min = true;
                    });
                    
                    let cancel_keyboard = make_cancel_keyboard(&state, chat_id);
                    bot.send_message(chat_id, t("enter_min_temp", lang))
                        .reply_markup(cancel_keyboard)
                        .await?;
                }
                Some(AlertType::WindSpeed { .. }) => {
                    update_user_data(&state, chat_id, |user_data| {
                        user_data.waiting_for_alert_wind_speed = true;
                    });
                    
                    let cancel_keyboard = make_cancel_keyboard(&state, chat_id);
                    bot.send_message(chat_id, t("enter_max_wind", lang))
                        .reply_markup(cancel_keyboard)
                        .await?;
                }
                Some(AlertType::Humidity { .. }) => {
                    update_user_data(&state, chat_id, |user_data| {
                        user_data.waiting_for_alert_humidity_min = true;
                    });
                    
                    let cancel_keyboard = make_cancel_keyboard(&state, chat_id);
                    bot.send_message(chat_id, t("enter_min_humidity", lang))
                        .reply_markup(cancel_keyboard)
                        .await?;
                }
                _ => {
                    bot.send_message(chat_id, t("error_unknown_alert", lang))
                        .await?;
                    
                    let keyboard = make_alerts_menu_keyboard(&state, chat_id);
                    bot.send_message(chat_id, t("alerts_management", lang))
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
                        bot.send_message(chat_id, t("invalid_temp", lang))
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
            
            let cancel_keyboard = make_cancel_keyboard(&state, chat_id);
            bot.send_message(chat_id, t("enter_max_temp", lang))
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
                        bot.send_message(chat_id, t("invalid_temp", lang))
                            .await?;
                        return Ok(());
                    }
                }
            };
            
            update_user_data(&state, chat_id, |user_data| {
                user_data.waiting_for_alert_temperature_max = false;
                user_data.waiting_for_alert_hours_input = true;
                if let Some(AlertType::TemperatureThreshold { max, .. }) = &mut user_data.pending_alert_type {
                    *max = max_temp;
                }
            });
            
            let cancel_keyboard = make_cancel_keyboard(&state, chat_id);
            bot.send_message(chat_id, t("hours_temp_warning", lang))
                .reply_markup(cancel_keyboard)
                .await?;
        }
        // Handle wind speed alert
        else if user_data.waiting_for_alert_wind_speed {
            match text.parse::<f32>() {
                Ok(wind_speed) => {
                    update_user_data(&state, chat_id, |user_data| {
                        user_data.waiting_for_alert_wind_speed = false;
                        user_data.waiting_for_alert_hours_input = true;
                        if let Some(AlertType::WindSpeed { max }) = &mut user_data.pending_alert_type {
                            *max = wind_speed;
                        }
                    });
                    
                    let cancel_keyboard = make_cancel_keyboard(&state, chat_id);
                    bot.send_message(chat_id, t("hours_wind_warning", lang))
                        .reply_markup(cancel_keyboard)
                        .await?;
                }
                Err(_) => {
                    bot.send_message(chat_id, t("invalid_wind", lang))
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
                        bot.send_message(chat_id, t("invalid_humidity", lang))
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
            
            let cancel_keyboard = make_cancel_keyboard(&state, chat_id);
            bot.send_message(chat_id, t("enter_max_humidity", lang))
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
                        bot.send_message(chat_id, t("invalid_humidity", lang))
                            .await?;
                        return Ok(());
                    }
                }
            };
            
            update_user_data(&state, chat_id, |user_data| {
                user_data.waiting_for_alert_humidity_max = false;
                user_data.waiting_for_alert_hours_input = true;
                if let Some(AlertType::Humidity { max, .. }) = &mut user_data.pending_alert_type {
                    *max = max_humidity;
                }
            });
            
            let cancel_keyboard = make_cancel_keyboard(&state, chat_id);
            bot.send_message(chat_id, t("hours_humidity_warning", lang))
                .reply_markup(cancel_keyboard)
                .await?;
        }
        // Handle hours input for alerts
        else if user_data.waiting_for_alert_hours_input {
            match text.parse::<u8>() {
                Ok(hours) if hours >= 1 && hours <= 72 => {
                    let user_data_clone = get_user_data(&state, chat_id);
                    if let (Some(city), Some(alert_type)) = (&user_data_clone.pending_alert_city, &user_data_clone.pending_alert_type) {
                        let alert = match alert_type {
                            AlertType::StandardWeatherAlert => create_standard_alert(city.clone(), hours),
                            AlertType::TemperatureThreshold { min, max } => create_temperature_alert(city.clone(), *min, *max, hours),
                            AlertType::WindSpeed { max } => create_wind_alert(city.clone(), *max, hours),
                            AlertType::Humidity { min, max } => create_humidity_alert(city.clone(), *min, *max, hours),
                        };
                        
                        update_user_data(&state, chat_id, |user_data| {
                            user_data.weather_alerts.push(alert);
                            user_data.waiting_for_alert_hours_input = false;
                            user_data.pending_alert_city = None;
                            user_data.pending_alert_type = None;
                            user_data.pending_alert_hours = None;
                        });
                        
                        let message = format!("{} '{}' {}", 
                            t("alert_created", lang), city, 
                            t("with_hours_warning", lang).replace("{}", &hours.to_string()));
                        bot.send_message(chat_id, message)
                            .await?;
                        
                        let keyboard = make_alerts_menu_keyboard(&state, chat_id);
                        bot.send_message(chat_id, t("alerts_management", lang))
                            .reply_markup(keyboard)
                            .await?;
                    } else {
                        bot.send_message(chat_id, t("error_no_pending_alert", lang))
                            .await?;
                    }
                }
                Ok(_) => {
                    bot.send_message(chat_id, t("invalid_hours", lang))
                        .await?;
                }
                Err(_) => {
                    bot.send_message(chat_id, t("invalid_number", lang))
                        .await?;
                }
            }
        }
    }
    
    Ok(())
}

pub fn make_main_menu_keyboard(state: &SharedState, chat_id: ChatId) -> InlineKeyboardMarkup {
    let lang = get_user_data(state, chat_id).language;
    let mut keyboard = vec![];

    keyboard.push(vec![InlineKeyboardButton::callback(
        t("weather", lang),
        "current_weather_menu",
    )]);
    
    keyboard.push(vec![InlineKeyboardButton::callback(
        t("forecast", lang), 
        "forecast_menu"
    )]);
    
    keyboard.push(vec![InlineKeyboardButton::callback(t("interested_towns", lang), "my_towns")]);
    
    keyboard.push(vec![InlineKeyboardButton::callback(t("alerts", lang), "alerts_menu")]);
    
    keyboard.push(vec![InlineKeyboardButton::callback(t("settings", lang), "settings_menu")]);

    InlineKeyboardMarkup::new(keyboard)
}

pub fn make_my_towns_keyboard(state: &SharedState, chat_id: ChatId) -> InlineKeyboardMarkup {
    let lang = get_user_data(state, chat_id).language;
    let mut keyboard = vec![];
    let user_data = get_user_data(state, chat_id);
    
    // Home town section
    if let Some(home_town) = &user_data.home_town {
        keyboard.push(vec![InlineKeyboardButton::callback(
            &format!("{} {} {}", t("home_prefix", lang), home_town, t("view_weather", lang)),
            "view_home_weather",
        )]);
        keyboard.push(vec![InlineKeyboardButton::callback(
            t("change_home_town", lang),
            "set_home_town",
        )]);
    } else {
        keyboard.push(vec![InlineKeyboardButton::callback(
            t("set_home_town", lang),
            "set_home_town",
        )]);
    }
    
    // Interested towns section
    if !user_data.interested_towns.is_empty() {
        keyboard.push(vec![InlineKeyboardButton::callback(
            t("interested_towns_separator", lang),
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
        t("add_interested_town", lang),
        "add_interested_town",
    )]);
    
    if !user_data.interested_towns.is_empty() {
        keyboard.push(vec![InlineKeyboardButton::callback(
            t("remove_interested_town", lang),
            "remove_interested_town",
        )]);
    }
    
    keyboard.push(vec![InlineKeyboardButton::callback(
        t("back_to_main", lang),
        "back_to_main",
    )]);

    InlineKeyboardMarkup::new(keyboard)
}

pub fn make_remove_towns_keyboard(state: &SharedState, chat_id: ChatId) -> InlineKeyboardMarkup {
    let lang = get_user_data(state, chat_id).language;
    let mut keyboard = vec![];
    let user_data = get_user_data(state, chat_id);
    
    if !user_data.interested_towns.is_empty() {
        keyboard.push(vec![InlineKeyboardButton::callback(
            t("select_town_separator", lang),
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
        t("back_to_interested_towns", lang),
        "back_to_interested_towns",
    )]);

    InlineKeyboardMarkup::new(keyboard)
}

pub fn make_cancel_keyboard(state: &SharedState, chat_id: ChatId) -> InlineKeyboardMarkup {
    let lang = get_user_data(state, chat_id).language;
    let keyboard = vec![
        vec![InlineKeyboardButton::callback(t("cancel", lang), "cancel")],
    ];

    InlineKeyboardMarkup::new(keyboard)
}

pub fn make_current_weather_keyboard(state: &SharedState, chat_id: ChatId) -> InlineKeyboardMarkup {
    let lang = get_user_data(state, chat_id).language;
    let mut keyboard = vec![];

    keyboard.push(vec![InlineKeyboardButton::callback(
        t("weather_for_city", lang),
        "get_weather_for",
    )]);
    
    keyboard.push(vec![InlineKeyboardButton::callback(
        t("weather_home", lang),
        "get_weather_home",
    )]);
    
    keyboard.push(vec![InlineKeyboardButton::callback(
        t("back_to_main", lang),
        "back_to_main",
    )]);

    InlineKeyboardMarkup::new(keyboard)
}

pub fn make_forecast_keyboard(state: &SharedState, chat_id: ChatId) -> InlineKeyboardMarkup {
    let lang = get_user_data(state, chat_id).language;
    let mut keyboard = vec![];

    keyboard.push(vec![InlineKeyboardButton::callback(
        t("forecast_for_city", lang),
        "get_forecast_for",
    )]);
    
    keyboard.push(vec![InlineKeyboardButton::callback(
        t("forecast_home", lang),
        "get_forecast_home",
    )]);
    
    keyboard.push(vec![InlineKeyboardButton::callback(
        t("back_to_main", lang),
        "back_to_main",
    )]);

    InlineKeyboardMarkup::new(keyboard)
}

pub fn make_alerts_menu_keyboard(state: &SharedState, chat_id: ChatId) -> InlineKeyboardMarkup {
    let lang = get_user_data(state, chat_id).language;
    let mut keyboard = vec![];
    let user_data = get_user_data(state, chat_id);
    
    // Показываем список алертов, если они есть
    if !user_data.weather_alerts.is_empty() {
        keyboard.push(vec![InlineKeyboardButton::callback(
            t("your_alerts_separator", lang),
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
            
            let alert_type_name = match &alert.alert_type {
                AlertType::StandardWeatherAlert => t("alert_type_standard", lang),
                AlertType::TemperatureThreshold { .. } => t("alert_type_temperature", lang),
                AlertType::WindSpeed { .. } => t("alert_type_wind", lang),
                AlertType::Humidity { .. } => t("alert_type_humidity", lang),
            };
            
            keyboard.push(vec![InlineKeyboardButton::callback(
                &format!("{} {} - {} {}", alert_type_emoji, alert.city, alert_type_name, status),
                &format!("check_alert_{}", alert.id),
            )]);
        }
    }
    
    // Кнопки управления
    keyboard.push(vec![InlineKeyboardButton::callback(
        t("add_alert", lang),
        "add_alert",
    )]);
    
    if !user_data.weather_alerts.is_empty() {
        keyboard.push(vec![InlineKeyboardButton::callback(
            t("remove_alert", lang),
            "remove_alert",
        )]);
    }
    
    keyboard.push(vec![InlineKeyboardButton::callback(
        t("back_to_main", lang),
        "back_to_main",
    )]);

    InlineKeyboardMarkup::new(keyboard)
}

pub fn make_add_alert_keyboard(state: &SharedState, chat_id: ChatId) -> InlineKeyboardMarkup {
    let lang = get_user_data(state, chat_id).language;
    let mut keyboard = vec![];
    
    keyboard.push(vec![InlineKeyboardButton::callback(
        t("standard_alert", lang),
        "add_standard_alert",
    )]);
    
    keyboard.push(vec![InlineKeyboardButton::callback(
        t("temperature_alert", lang),
        "add_temperature_alert",
    )]);
    
    keyboard.push(vec![InlineKeyboardButton::callback(
        t("wind_alert", lang),
        "add_wind_alert",
    )]);
    
    keyboard.push(vec![InlineKeyboardButton::callback(
        t("humidity_alert", lang),
        "add_humidity_alert",
    )]);
    
    keyboard.push(vec![InlineKeyboardButton::callback(
        t("back_to_alerts", lang),
        "alerts_menu",
    )]);

    InlineKeyboardMarkup::new(keyboard)
}

pub fn make_remove_alerts_keyboard(state: &SharedState, chat_id: ChatId) -> InlineKeyboardMarkup {
    let lang = get_user_data(state, chat_id).language;
    let mut keyboard = vec![];
    let user_data = get_user_data(state, chat_id);
    
    if !user_data.weather_alerts.is_empty() {
        keyboard.push(vec![InlineKeyboardButton::callback(
            t("select_alert_separator", lang),
            "noop",
        )]);
        
        for alert in &user_data.weather_alerts {
            let alert_type_str = match &alert.alert_type {
                AlertType::StandardWeatherAlert => "🚨",
                AlertType::TemperatureThreshold { .. } => "🌡️",
                AlertType::WindSpeed { .. } => "💨",
                AlertType::Humidity { .. } => "💧",
            };
            
            let alert_type_name = match &alert.alert_type {
                AlertType::StandardWeatherAlert => t("alert_type_standard", lang),
                AlertType::TemperatureThreshold { .. } => t("alert_type_temperature", lang),
                AlertType::WindSpeed { .. } => t("alert_type_wind", lang),
                AlertType::Humidity { .. } => t("alert_type_humidity", lang),
            };
            
            let status_text = if alert.is_active { 
                t("active", lang) 
            } else { 
                t("inactive", lang) 
            };
            
            let button_text = format!("{} {} - {} ({})", alert_type_str, alert.city, 
                                    alert_type_name, status_text);
            
            keyboard.push(vec![InlineKeyboardButton::callback(
                &button_text,
                &format!("remove_alert_{}", alert.id),
            )]);
        }
    }
    
    keyboard.push(vec![InlineKeyboardButton::callback(
        t("back_to_alerts", lang),
        "alerts_menu",
    )]);

    InlineKeyboardMarkup::new(keyboard)
}

pub fn make_language_selection_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard = vec![];
    
    keyboard.push(vec![InlineKeyboardButton::callback(
        "🇺🇸 English",
        "language_en",
    )]);
    
    keyboard.push(vec![InlineKeyboardButton::callback(
        "🇷🇺 Русский",
        "language_ru",
    )]);
    
    keyboard.push(vec![InlineKeyboardButton::callback(
        "← Back to Main Menu",
        "back_to_main",
    )]);

    InlineKeyboardMarkup::new(keyboard)
}

pub fn make_settings_keyboard(state: &SharedState, chat_id: ChatId) -> InlineKeyboardMarkup {
    let lang = get_user_data(state, chat_id).language;
    let mut keyboard = vec![];
    
    keyboard.push(vec![InlineKeyboardButton::callback(
        t("change_language", lang),
        "change_language",
    )]);
    
    keyboard.push(vec![InlineKeyboardButton::callback(
        t("back_to_main", lang),
        "back_to_main",
    )]);

    InlineKeyboardMarkup::new(keyboard)
}

 