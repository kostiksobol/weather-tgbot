use teloxide::types::{InlineKeyboardButtonKind, Message};
use teloxide_tests::{MockBot, MockCallbackQuery, MockMessageText, MockUser};
use weather_tgbot::{initialize_bot, handler_tree, state::create_test_shared_state};
use std::fs;

#[tokio::main]
async fn main() {
    // Используем ту же инициализацию, что и реальный бот
    initialize_bot();
    
    println!("--- Running Bot Simulation ---");

    // === SETUP ===
    let alice = MockUser::new().id(123).first_name("Alice").build();
    let shared_state = create_test_shared_state().expect("Failed to create test shared state");
    let mut bot = MockBot::new(MockMessageText::new(), handler_tree(shared_state));

    // === TEST /START COMMAND ===
    println!("\n--> User sends: /start");
    bot.update(MockMessageText::new().text("/start").from(alice.clone()));
    bot.dispatch().await;

    let start_responses = bot.get_responses();
    let start_message = start_responses
        .sent_messages
        .last()
        .expect("/start should send a message");

    println!("Response: {}", start_message.text().unwrap_or("(no text)"));
    print_buttons(start_message);

    // === TEST CURRENT WEATHER FLOW - LONDON ===
    println!("\n=== TESTING CURRENT WEATHER FOR LONDON ===");
    
    println!("\n--> User presses button: Current weather");
    let weather_callback = MockCallbackQuery::new()
        .data("current_weather_menu")
        .message(start_message.clone());
    bot.update(weather_callback);
    bot.dispatch().await;

    let weather_responses = bot.get_responses();
    if let Some(message) = weather_responses.sent_messages.last() {
        println!("Current weather menu: {}", message.text().unwrap_or("(no text)"));
        print_buttons(message);
    }

    println!("\n--> User presses button: For any city");
    let any_city_callback = MockCallbackQuery::new()
        .data("get_weather_for")
        .message(start_message.clone());
    bot.update(any_city_callback);
    bot.dispatch().await;

    let any_city_responses = bot.get_responses();
    if let Some(message) = any_city_responses.sent_messages.last() {
        println!("Enter city prompt: {}", message.text().unwrap_or("(no text)"));
    }

    println!("\n--> User sends: London");
    bot.update(MockMessageText::new().text("London").from(alice.clone()));
    bot.dispatch().await;

    let london_responses = bot.get_responses();
    print_current_weather_response(&london_responses, "London");

    // === TEST FORECAST FLOW - TOKYO ===
    println!("\n=== TESTING FORECAST FOR TOKYO ===");
    
    println!("\n--> User presses button: Forecast");
    let forecast_callback = MockCallbackQuery::new()
        .data("forecast_menu")
        .message(start_message.clone());
    bot.update(forecast_callback);
    bot.dispatch().await;

    let forecast_responses = bot.get_responses();
    if let Some(message) = forecast_responses.sent_messages.last() {
        println!("Forecast menu: {}", message.text().unwrap_or("(no text)"));
        print_buttons(message);
    }

    println!("\n--> User presses button: For any city");
    let forecast_any_city_callback = MockCallbackQuery::new()
        .data("get_forecast_for")
        .message(start_message.clone());
    bot.update(forecast_any_city_callback);
    bot.dispatch().await;

    let forecast_any_city_responses = bot.get_responses();
    if let Some(message) = forecast_any_city_responses.sent_messages.last() {
        println!("Enter city for forecast: {}", message.text().unwrap_or("(no text)"));
    }

    println!("\n--> User sends: Tokyo");
    bot.update(MockMessageText::new().text("Tokyo").from(alice.clone()));
    bot.dispatch().await;

    let tokyo_responses = bot.get_responses();
    print_forecast_response(&tokyo_responses, "Tokyo");

    // === TEST FORECAST FOR HOME ===
    println!("\n=== TESTING FORECAST FOR HOME (should be set to Paris) ===");
    
    println!("\n--> User presses button: Forecast");
    let forecast_home_callback = MockCallbackQuery::new()
        .data("forecast_menu")
        .message(start_message.clone());
    bot.update(forecast_home_callback);
    bot.dispatch().await;

    let forecast_home_responses = bot.get_responses();
    if let Some(message) = forecast_home_responses.sent_messages.last() {
        println!("Forecast menu: {}", message.text().unwrap_or("(no text)"));
    }

    println!("\n--> User presses button: For home");
    let forecast_for_home_callback = MockCallbackQuery::new()
        .data("get_forecast_home")
        .message(start_message.clone());
    bot.update(forecast_for_home_callback);
    bot.dispatch().await;

    let forecast_for_home_responses = bot.get_responses();
    if let Some(message) = forecast_for_home_responses.sent_messages.last() {
        println!("Home forecast response: {}", message.text().unwrap_or("(no text)"));
    }

    // === TEST INVALID CITY ===
    println!("\n=== TESTING WITH INVALID CITY ===");
    
    println!("\n--> User presses button: Current weather");
    let weather_callback3 = MockCallbackQuery::new()
        .data("current_weather_menu")
        .message(start_message.clone());
    bot.update(weather_callback3);
    bot.dispatch().await;

    let weather_responses3 = bot.get_responses();
    if let Some(message) = weather_responses3.sent_messages.last() {
        println!("Current weather menu: {}", message.text().unwrap_or("(no text)"));
    }

    println!("\n--> User presses button: For any city");
    let invalid_any_city_callback = MockCallbackQuery::new()
        .data("get_weather_for")
        .message(start_message.clone());
    bot.update(invalid_any_city_callback);
    bot.dispatch().await;

    let invalid_any_city_responses = bot.get_responses();
    if let Some(message) = invalid_any_city_responses.sent_messages.last() {
        println!("Enter city prompt: {}", message.text().unwrap_or("(no text)"));
    }

    println!("\n--> User sends: InvalidCityName123");
    bot.update(MockMessageText::new().text("InvalidCityName123").from(alice.clone()));
    bot.dispatch().await;

    let invalid_responses = bot.get_responses();
    if let Some(message) = invalid_responses.sent_messages.last() {
        println!("Invalid city response: {}", message.text().unwrap_or("(no text)"));
    }

    // === TEST OTHER BUTTONS ===
    println!("\n=== TESTING OTHER BUTTONS ===");
    
    // Test Interested towns button
    println!("\n--> User presses button: Interested towns");
    let callback2 = MockCallbackQuery::new()
        .data("my_towns")
        .message(start_message.clone());
    bot.update(callback2);
    bot.dispatch().await;

    let responses2 = bot.get_responses();
    if let Some(message) = responses2.sent_messages.last() {
        println!("Interested towns response: {}", message.text().unwrap_or("(no text)"));
        print_buttons(message);
    }
    
    // === TEST MY TOWNS FUNCTIONALITY ===
    println!("\n=== TESTING MY TOWNS FUNCTIONALITY ===");
    
    // Test setting home town
    println!("\n--> User presses button: Set Home Town");
    let set_home_callback = MockCallbackQuery::new()
        .data("set_home_town")
        .message(start_message.clone());
    bot.update(set_home_callback);
    bot.dispatch().await;

    let set_home_responses = bot.get_responses();
    if let Some(message) = set_home_responses.sent_messages.last() {
        println!("Set Home Town response: {}", message.text().unwrap_or("(no text)"));
    }
    
    println!("\n--> User sends: Paris");
    bot.update(MockMessageText::new().text("Paris").from(alice.clone()));
    bot.dispatch().await;

    let paris_responses = bot.get_responses();
    if let Some(message) = paris_responses.sent_messages.last() {
        println!("Home town set response: {}", message.text().unwrap_or("(no text)"));
        print_buttons(message);
    }
    
    // Test viewing home weather
    println!("\n--> User presses button: Current weather -> For home");
    let current_weather_callback = MockCallbackQuery::new()
        .data("current_weather_menu")
        .message(start_message.clone());
    bot.update(current_weather_callback);
    bot.dispatch().await;

    let current_weather_responses = bot.get_responses();
    if let Some(message) = current_weather_responses.sent_messages.last() {
        println!("Current weather menu: {}", message.text().unwrap_or("(no text)"));
        print_buttons(message);
    }

    println!("\n--> User presses button: For home");
    let for_home_callback = MockCallbackQuery::new()
        .data("get_weather_home")
        .message(start_message.clone());
    bot.update(for_home_callback);
    bot.dispatch().await;

    let home_weather_responses = bot.get_responses();
    if let Some(message) = home_weather_responses.sent_messages.last() {
        println!("Home weather response: {}", message.text().unwrap_or("(no text)"));
    }
    
    // Test adding interested town
    println!("\n--> User presses button: Add Interested Town");
    let add_town_callback = MockCallbackQuery::new()
        .data("add_interested_town")
        .message(start_message.clone());
    bot.update(add_town_callback);
    bot.dispatch().await;

    let add_town_responses = bot.get_responses();
    if let Some(message) = add_town_responses.sent_messages.last() {
        println!("Add interested town response: {}", message.text().unwrap_or("(no text)"));
    }
    
    println!("\n--> User sends: Berlin");
    bot.update(MockMessageText::new().text("Berlin").from(alice.clone()));
    bot.dispatch().await;

    let berlin_responses = bot.get_responses();
    if let Some(message) = berlin_responses.sent_messages.last() {
        println!("Berlin added response: {}", message.text().unwrap_or("(no text)"));
        print_buttons(message);
    }
    
    // Test clicking on interested town to get weather
    println!("\n--> User presses button: Berlin (interested town)");
    let berlin_weather_callback = MockCallbackQuery::new()
        .data("town_Berlin")
        .message(start_message.clone());
    bot.update(berlin_weather_callback);
    bot.dispatch().await;

    let berlin_weather_responses = bot.get_responses();
    if let Some(message) = berlin_weather_responses.sent_messages.last() {
        println!("Berlin weather response: {}", message.text().unwrap_or("(no text)"));
    }

    // === TEST REMOVING INTERESTED TOWNS ===
    println!("\n=== TESTING REMOVING INTERESTED TOWNS ===");
    
    // First, go back to Interested towns
    println!("\n--> User presses button: Interested towns");
    let my_towns_callback = MockCallbackQuery::new()
        .data("my_towns")
        .message(start_message.clone());
    bot.update(my_towns_callback);
    bot.dispatch().await;

    let my_towns_responses = bot.get_responses();
    if let Some(message) = my_towns_responses.sent_messages.last() {
        println!("Interested towns menu: {}", message.text().unwrap_or("(no text)"));
        print_buttons(message);
    }
    
    println!("\n--> User presses button: Remove Interested Town");
    let remove_town_callback = MockCallbackQuery::new()
        .data("remove_interested_town")
        .message(start_message.clone());
    bot.update(remove_town_callback);
    bot.dispatch().await;

    let remove_town_responses = bot.get_responses();
    if let Some(message) = remove_town_responses.sent_messages.last() {
        println!("Remove town menu: {}", message.text().unwrap_or("(no text)"));
        print_buttons(message);
    }
    
    println!("\n--> User presses button: Remove Berlin");
    let remove_berlin_callback = MockCallbackQuery::new()
        .data("remove_town_Berlin")
        .message(start_message.clone());
    bot.update(remove_berlin_callback);
    bot.dispatch().await;

    let remove_berlin_responses = bot.get_responses();
    if let Some(message) = remove_berlin_responses.sent_messages.last() {
        println!("Berlin removed response: {}", message.text().unwrap_or("(no text)"));
        print_buttons(message);
    }
    
    // === TEST ADDING MULTIPLE INTERESTED TOWNS ===
    println!("\n=== TESTING MULTIPLE INTERESTED TOWNS ===");
    
    println!("\n--> User presses button: Add Interested Town");
    let add_town2_callback = MockCallbackQuery::new()
        .data("add_interested_town")
        .message(start_message.clone());
    bot.update(add_town2_callback);
    bot.dispatch().await;

    let add_town2_responses = bot.get_responses();
    if let Some(message) = add_town2_responses.sent_messages.last() {
        println!("Add town response: {}", message.text().unwrap_or("(no text)"));
    }
    
    println!("\n--> User sends: New York");
    bot.update(MockMessageText::new().text("New York").from(alice.clone()));
    bot.dispatch().await;

    let ny_responses = bot.get_responses();
    if let Some(message) = ny_responses.sent_messages.last() {
        println!("New York added: {}", message.text().unwrap_or("(no text)"));
        print_buttons(message);
    }
    
    println!("\n--> User presses button: Add Interested Town (second city)");
    let add_town3_callback = MockCallbackQuery::new()
        .data("add_interested_town")
        .message(start_message.clone());
    bot.update(add_town3_callback);
    bot.dispatch().await;

    let add_town3_responses = bot.get_responses();
    if let Some(message) = add_town3_responses.sent_messages.last() {
        println!("Add town response: {}", message.text().unwrap_or("(no text)"));
    }
    
    println!("\n--> User sends: Sydney");
    bot.update(MockMessageText::new().text("Sydney").from(alice.clone()));
    bot.dispatch().await;

    let sydney_responses = bot.get_responses();
    if let Some(message) = sydney_responses.sent_messages.last() {
        println!("Sydney added: {}", message.text().unwrap_or("(no text)"));
        print_buttons(message);
    }
    
    // === TEST CHANGING HOME TOWN ===
    println!("\n=== TESTING CHANGING HOME TOWN ===");
    
    println!("\n--> User presses button: Change Home Town");
    let change_home_callback = MockCallbackQuery::new()
        .data("set_home_town")
        .message(start_message.clone());
    bot.update(change_home_callback);
    bot.dispatch().await;

    let change_home_responses = bot.get_responses();
    if let Some(message) = change_home_responses.sent_messages.last() {
        println!("Change home town response: {}", message.text().unwrap_or("(no text)"));
    }
    
    println!("\n--> User sends: Amsterdam");
    bot.update(MockMessageText::new().text("Amsterdam").from(alice.clone()));
    bot.dispatch().await;

    let amsterdam_responses = bot.get_responses();
    if let Some(message) = amsterdam_responses.sent_messages.last() {
        println!("Amsterdam set as home: {}", message.text().unwrap_or("(no text)"));
        print_buttons(message);
    }

    // Test Alerts button
    println!("\n--> User presses button: Alerts");
    let callback3 = MockCallbackQuery::new()
        .data("alerts")
        .message(start_message.clone());
    bot.update(callback3);
    bot.dispatch().await;

    let responses3 = bot.get_responses();
    if let Some(message) = responses3.sent_messages.last() {
        println!("Alerts response: {}", message.text().unwrap_or("(no text)"));
    }

    // === TEST /HELP COMMAND ===
    println!("\n--> User sends: /help");
    bot.update(MockMessageText::new().text("/help").from(alice.clone()));
    bot.dispatch().await;

    let help_responses = bot.get_responses();
    if let Some(message) = help_responses.sent_messages.last() {
        println!("Help response: {}", message.text().unwrap_or("(no text)"));
    }

    println!("\n--- All tests finished ---");

    // === TEST DATA CLEANUP ON /START ===
    println!("\n=== TESTING DATA CLEANUP ON /START ===");
    
    // First, verify user has some data (home town and interested towns)
    println!("\n--> User presses button: Interested towns (should show Amsterdam as home and Sydney, New York as interested)");
    let final_towns_callback = MockCallbackQuery::new()
        .data("my_towns")
        .message(start_message.clone());
    bot.update(final_towns_callback);
    bot.dispatch().await;

    let final_towns_responses = bot.get_responses();
    if let Some(message) = final_towns_responses.sent_messages.last() {
        println!("Current user data: {}", message.text().unwrap_or("(no text)"));
        print_buttons(message);
    }

    // NOW TEST /start CLEANUP
    println!("\n--> User sends: /start (should clear all data)");
    bot.update(MockMessageText::new().text("/start").from(alice.clone()));
    bot.dispatch().await;

    let cleanup_responses = bot.get_responses();
    if let Some(message) = cleanup_responses.sent_messages.last() {
        println!("Response after /start: {}", message.text().unwrap_or("(no text)"));
        print_buttons(message);
    }

    // Verify data is actually cleared
    println!("\n--> User presses button: Interested towns (should show no home town or interested towns)");
    let empty_towns_callback = MockCallbackQuery::new()
        .data("my_towns")
        .message(start_message.clone());
    bot.update(empty_towns_callback);
    bot.dispatch().await;

    let empty_towns_responses = bot.get_responses();
    if let Some(message) = empty_towns_responses.sent_messages.last() {
        println!("User data after /start cleanup: {}", message.text().unwrap_or("(no text)"));
        print_buttons(message);
    }

    // Test home weather button after cleanup
    println!("\n--> User presses button: Current weather -> For home (should ask to set home town)");
    let cleanup_weather_callback = MockCallbackQuery::new()
        .data("current_weather_menu")
        .message(start_message.clone());
    bot.update(cleanup_weather_callback);
    bot.dispatch().await;

    let cleanup_weather_responses = bot.get_responses();
    if let Some(message) = cleanup_weather_responses.sent_messages.last() {
        print_buttons(message);
    }

    println!("\n--> User presses button: For home");
    let cleanup_home_callback = MockCallbackQuery::new()
        .data("get_weather_home")
        .message(start_message.clone());
    bot.update(cleanup_home_callback);
    bot.dispatch().await;

    let cleanup_home_responses = bot.get_responses();
    if let Some(message) = cleanup_home_responses.sent_messages.last() {
        println!("Response when no home town set: {}", message.text().unwrap_or("(no text)"));
    }

    println!("\n=== DATA CLEANUP TEST COMPLETED ===");
    println!("✅ /start command should clear all user data for fresh start");
    
    println!("\n--- All tests finished ---");
    
    // === CLEANUP TEST DATABASE FILES ===
    println!("\n🧹 Cleaning up test database files...");
    cleanup_test_databases();
    println!("✅ Test cleanup completed");
}

/// Clean up test database files after tests
fn cleanup_test_databases() {
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(name) = path.file_name() {
                    if let Some(name_str) = name.to_str() {
                        if name_str.starts_with("test_weather_bot_data_") {
                            if path.is_dir() {
                                if let Err(e) = fs::remove_dir_all(&path) {
                                    println!("❌ Failed to remove {}: {}", name_str, e);
                                } else {
                                    println!("🗑️  Removed: {}", name_str);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Prints current weather response only
fn print_current_weather_response(responses: &teloxide_tests::Responses, city: &str) {
    println!("Bot responses after receiving '{}':", city);
    
    println!("\n--- Current Weather Response ---");
    for (i, message) in responses.sent_messages.iter().enumerate() {
        let text = message.text().unwrap_or("(no text)");
        if text.contains("🌍") || text.contains("🌡️") || text.contains("☁️") || text.contains("💨") || text.contains("💧") {
            println!("Weather data message:");
            println!("{}", text);
        } else if i == responses.sent_messages.len() - 1 {
            println!("Final menu message: {}", text);
        }
    }
}

/// Prints forecast response only  
fn print_forecast_response(responses: &teloxide_tests::Responses, city: &str) {
    println!("Bot responses after receiving '{}':", city);
    
    println!("\n--- 3-Day Forecast Response ---");
    for (i, message) in responses.sent_messages.iter().enumerate() {
        let text = message.text().unwrap_or("(no text)");
        if text.contains("📅") || text.contains("📆") || text.contains("🌡️") || text.contains("☁️") || text.contains("💨") || text.contains("💧") {
            println!("Forecast data message:");
            println!("{}", text);
        } else if i == responses.sent_messages.len() - 1 {
            println!("Final menu message: {}", text);
        }
    }
}

/// Prints inline keyboard buttons from a message
fn print_buttons(message: &Message) {
    if let Some(markup) = message.reply_markup() {
        println!("  Buttons:");
        for row in &markup.inline_keyboard {
            for button in row {
                if let InlineKeyboardButtonKind::CallbackData(data) = &button.kind {
                    println!("    - '{}' (Action: {})", button.text, data);
                }
            }
        }
    }
}
