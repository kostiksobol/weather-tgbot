use teloxide::types::{InlineKeyboardButtonKind, Message};
use teloxide_tests::{MockBot, MockCallbackQuery, MockMessageText, MockUser};
use weather_tgbot::{initialize_bot, handler_tree, state::create_test_shared_state};
use std::fs;

#[tokio::main]
async fn main() {
    initialize_bot();
    
    println!("--- Running Bot Simulation from README (Corrected Flow) ---");

    let user = MockUser::new().id(123).first_name("TestUser").build();
    let shared_state = create_test_shared_state().expect("Failed to create test shared_state");
    let mut bot = MockBot::new(MockMessageText::new(), handler_tree(shared_state));

    // 1. USER: START BOT (/start)
    println!("\n--> User sends: /start");
    bot.update(MockMessageText::new().text("/start").from(user.clone()));
    bot.dispatch().await;
    let start_responses = bot.get_responses();
    let mut last_message = start_responses.sent_messages.last().expect("/start should send a message").clone();
    println!("Response: {}", last_message.text().unwrap_or("(no text)"));
    print_buttons(&last_message);

    // 2. USER: SELECTS "Interested towns"
    println!("\n--> User presses button: Interested towns");
    let towns_callback = MockCallbackQuery::new().data("my_towns").message(last_message.clone());
    bot.update(towns_callback);
    bot.dispatch().await;
    last_message = bot.get_responses().sent_messages.last().unwrap().clone();
    println!("Response: {}", last_message.text().unwrap_or("(no text)"));
    print_buttons(&last_message);

    // 3. USER: SELECTS "Set Home Town"
    println!("\n--> User presses button: Set Home Town");
    let set_home_callback = MockCallbackQuery::new().data("set_home_town").message(last_message.clone());
    bot.update(set_home_callback);
    bot.dispatch().await;
    last_message = bot.get_responses().sent_messages.last().unwrap().clone();
    println!("Response: {}", last_message.text().unwrap());

    // 4. USER: SENDS "Kyiv"
    println!("\n--> User sends: Kyiv");
    bot.update(MockMessageText::new().text("Kyiv").from(user.clone()));
    bot.dispatch().await;
    let home_town_set_responses = bot.get_responses();
    // Bot sends two messages: confirmation and new menu
    println!("Response: {}", home_town_set_responses.sent_messages.get(0).unwrap().text().unwrap());
    last_message = home_town_set_responses.sent_messages.get(1).unwrap().clone();
    println!("Response: {}", last_message.text().unwrap());
    print_buttons(&last_message); // This should be the MAIN MENU

    // Corrected Flow: User must navigate back to "Interested towns" menu
    println!("\n--> User presses button: Interested towns (from main menu)");
    let towns_callback_2 = MockCallbackQuery::new().data("my_towns").message(last_message.clone());
    bot.update(towns_callback_2);
    bot.dispatch().await;
    last_message = bot.get_responses().sent_messages.last().unwrap().clone();
    println!("Response: {}", last_message.text().unwrap());
    print_buttons(&last_message); // This is now the "My Towns" menu

    // 5. USER: SELECTS "Add Interested Town"
    println!("\n--> User presses button: Add Interested Town");
    let add_interested_callback = MockCallbackQuery::new().data("add_interested_town").message(last_message.clone());
    bot.update(add_interested_callback);
    bot.dispatch().await;
    last_message = bot.get_responses().sent_messages.last().unwrap().clone();
    println!("Response: {}", last_message.text().unwrap());

    // 6. USER: SENDS "Lviv"
    println!("\n--> User sends: Lviv");
    bot.update(MockMessageText::new().text("Lviv").from(user.clone()));
    bot.dispatch().await;
    let lviv_added_responses = bot.get_responses();
    println!("Response: {}", lviv_added_responses.sent_messages.get(0).unwrap().text().unwrap());
    last_message = lviv_added_responses.sent_messages.get(1).unwrap().clone(); // This is the "My Towns" menu again
    println!("Response: {}", last_message.text().unwrap());
    print_buttons(&last_message);

    // 7. USER: Goes back to Main Menu to select "Current weather"
    println!("\n--> User presses button: ← Back to Main Menu");
    let back_to_main_callback = MockCallbackQuery::new().data("back_to_main").message(last_message.clone());
    bot.update(back_to_main_callback);
    bot.dispatch().await;
    last_message = bot.get_responses().sent_messages.last().unwrap().clone();
    println!("Response: {}", last_message.text().unwrap());
    print_buttons(&last_message);


    // 8. USER: SELECTS "Current weather"
    println!("\n--> User presses button: Current weather");
    let current_weather_callback = MockCallbackQuery::new().data("current_weather_menu").message(last_message.clone());
    bot.update(current_weather_callback);
    bot.dispatch().await;
    last_message = bot.get_responses().sent_messages.last().unwrap().clone();
    println!("Response: {}", last_message.text().unwrap());
    print_buttons(&last_message);

    // 9. USER: SELECTS "For home"
    println!("\n--> User presses button: For home");
    let home_weather_callback = MockCallbackQuery::new().data("get_weather_home").message(last_message.clone());
    bot.update(home_weather_callback);
    bot.dispatch().await;
    let home_weather_responses = bot.get_responses();
    print_weather_response(&home_weather_responses, "Kyiv");
    last_message = home_weather_responses.sent_messages.last().unwrap().clone(); // Back to main menu


    // 10. USER: SELECTS "Forecast"
    println!("\n--> User presses button: Forecast");
    let forecast_callback = MockCallbackQuery::new().data("forecast_menu").message(last_message.clone());
    bot.update(forecast_callback);
    bot.dispatch().await;
    last_message = bot.get_responses().sent_messages.last().unwrap().clone();
    println!("Response: {}", last_message.text().unwrap());
    print_buttons(&last_message);

    // 11. USER: SELECTS "For any city"
    println!("\n--> User presses button: For any city");
    let any_city_forecast_callback = MockCallbackQuery::new().data("get_forecast_for").message(last_message.clone());
    bot.update(any_city_forecast_callback);
    bot.dispatch().await;
    last_message = bot.get_responses().sent_messages.last().unwrap().clone();
    println!("Response: {}", last_message.text().unwrap());

    // 12. USER: SENDS "Odesa"
    println!("\n--> User sends: Odesa");
    bot.update(MockMessageText::new().text("Odesa").from(user.clone()));
    bot.dispatch().await;
    let odesa_forecast_responses = bot.get_responses();
    print_forecast_response(&odesa_forecast_responses, "Odesa");
    last_message = odesa_forecast_responses.sent_messages.last().unwrap().clone(); // Back to main menu


    // 13. USER: SELECTS "Weather Alerts"
    println!("\n--> User presses button: Weather Alerts");
    let alerts_menu_callback = MockCallbackQuery::new().data("alerts_menu").message(last_message.clone());
    bot.update(alerts_menu_callback);
    bot.dispatch().await;
    last_message = bot.get_responses().sent_messages.last().unwrap().clone();
    println!("Response: {}", last_message.text().unwrap());
    print_buttons(&last_message);

    // 14. USER: SELECTS "Add Alert"
    println!("\n--> User presses button: Add Alert");
    let add_alert_callback = MockCallbackQuery::new().data("add_alert").message(last_message.clone());
    bot.update(add_alert_callback);
    bot.dispatch().await;
    last_message = bot.get_responses().sent_messages.last().unwrap().clone();
    println!("Response: {}", last_message.text().unwrap());
    print_buttons(&last_message);

    // 15. USER: SELECTS "Temperature Alert"
    println!("\n--> User presses button: Temperature Alert");
    let temp_alert_callback = MockCallbackQuery::new().data("add_temperature_alert").message(last_message.clone());
    bot.update(temp_alert_callback);
    bot.dispatch().await;
    last_message = bot.get_responses().sent_messages.last().unwrap().clone();
    println!("Response: {}", last_message.text().unwrap());

    // 16. USER: SENDS "Kharkiv"
    println!("\n--> User sends: Kharkiv");
    bot.update(MockMessageText::new().text("Kharkiv").from(user.clone()));
    bot.dispatch().await;
    last_message = bot.get_responses().sent_messages.last().unwrap().clone();
    println!("Response: {}", last_message.text().unwrap());

    // 17. USER: SENDS "10" (min temp)
    println!("\n--> User sends: 10");
    bot.update(MockMessageText::new().text("10").from(user.clone()));
    bot.dispatch().await;
    last_message = bot.get_responses().sent_messages.last().unwrap().clone();
    println!("Response: {}", last_message.text().unwrap());

    // 18. USER: SENDS "30" (max temp)
    println!("\n--> User sends: 30");
    bot.update(MockMessageText::new().text("30").from(user.clone()));
    bot.dispatch().await;
    last_message = bot.get_responses().sent_messages.last().unwrap().clone();
    println!("Response: {}", last_message.text().unwrap());
    
    // 19. USER: SENDS "24" (hours)
    println!("\n--> User sends: 24");
    bot.update(MockMessageText::new().text("24").from(user.clone()));
    bot.dispatch().await;
    let alert_created_responses = bot.get_responses();
    // Bot sends two messages: confirmation and new menu
    println!("Response: {}", alert_created_responses.sent_messages.get(0).unwrap().text().unwrap());
    last_message = alert_created_responses.sent_messages.get(1).unwrap().clone();
    println!("Response: {}", last_message.text().unwrap());
    print_buttons(&last_message);


    println!("\n--- Simulation Finished ---");
    cleanup_test_databases();
}

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

fn print_weather_response(responses: &teloxide_tests::Responses, city: &str) {
    println!("Bot responses after getting weather for '{}':", city);
    for message in responses.sent_messages.iter() {
        let text = message.text().unwrap_or("(no text)");
        println!("{}", text);
    }
}

fn print_forecast_response(responses: &teloxide_tests::Responses, city: &str) {
    println!("Bot responses after getting forecast for '{}':", city);
    for message in responses.sent_messages.iter() {
        let text = message.text().unwrap_or("(no text)");
        println!("{}", text);
    }
}

fn cleanup_test_databases() {
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with("test_weather_bot_data_") && path.is_dir() {
                    if fs::remove_dir_all(&path).is_ok() {
                        println!("Removed test database: {}", name);
                    }
                }
            }
        }
    }
} 