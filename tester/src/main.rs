use teloxide::types::{InlineKeyboardButtonKind, Message};
use teloxide_tests::{MockBot, MockCallbackQuery, MockMessageText, MockUser};
use weather_tgbot::{initialize_bot, handler_tree, state::create_shared_state};

#[tokio::main]
async fn main() {
    // Используем ту же инициализацию, что и реальный бот
    initialize_bot();
    
    println!("--- Running Bot Simulation ---");

    // === SETUP ===
    let alice = MockUser::new().id(123).first_name("Alice").build();
    let shared_state = create_shared_state();
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

    // === TEST WEATHER FLOW - LONDON ===
    println!("\n=== TESTING WEATHER FOR LONDON ===");
    
    println!("\n--> User presses button: Get Weather");
    let weather_callback = MockCallbackQuery::new()
        .data("get_weather")
        .message(start_message.clone());
    bot.update(weather_callback);
    bot.dispatch().await;

    let weather_responses = bot.get_responses();
    if let Some(message) = weather_responses.sent_messages.last() {
        println!("Button response: {}", message.text().unwrap_or("(no text)"));
    }

    println!("\n--> User sends: London");
    bot.update(MockMessageText::new().text("London").from(alice.clone()));
    bot.dispatch().await;

    let london_responses = bot.get_responses();
    print_weather_responses(&london_responses, "London");

    // === TEST WEATHER FLOW - TOKYO ===
    println!("\n=== TESTING WEATHER FOR TOKYO ===");
    
    println!("\n--> User presses button: Get Weather");
    let weather_callback2 = MockCallbackQuery::new()
        .data("get_weather")
        .message(start_message.clone());
    bot.update(weather_callback2);
    bot.dispatch().await;

    let weather_responses2 = bot.get_responses();
    if let Some(message) = weather_responses2.sent_messages.last() {
        println!("Button response: {}", message.text().unwrap_or("(no text)"));
    }

    println!("\n--> User sends: Tokyo");
    bot.update(MockMessageText::new().text("Tokyo").from(alice.clone()));
    bot.dispatch().await;

    let tokyo_responses = bot.get_responses();
    print_weather_responses(&tokyo_responses, "Tokyo");

    // === TEST INVALID CITY ===
    println!("\n=== TESTING WITH INVALID CITY ===");
    
    println!("\n--> User presses button: Get Weather");
    let weather_callback3 = MockCallbackQuery::new()
        .data("get_weather")
        .message(start_message.clone());
    bot.update(weather_callback3);
    bot.dispatch().await;

    let weather_responses3 = bot.get_responses();
    if let Some(message) = weather_responses3.sent_messages.last() {
        println!("Button response: {}", message.text().unwrap_or("(no text)"));
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
    
    // Test My Towns button
    println!("\n--> User presses button: My Towns");
    let callback2 = MockCallbackQuery::new()
        .data("my_towns")
        .message(start_message.clone());
    bot.update(callback2);
    bot.dispatch().await;

    let responses2 = bot.get_responses();
    if let Some(message) = responses2.sent_messages.last() {
        println!("My Towns response: {}", message.text().unwrap_or("(no text)"));
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
    println!("\n--> User presses button: View Home Weather");
    let view_home_callback = MockCallbackQuery::new()
        .data("view_home_weather")
        .message(start_message.clone());
    bot.update(view_home_callback);
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
    
    // First, go back to My Towns
    println!("\n--> User presses button: My Towns");
    let my_towns_callback = MockCallbackQuery::new()
        .data("my_towns")
        .message(start_message.clone());
    bot.update(my_towns_callback);
    bot.dispatch().await;

    let my_towns_responses = bot.get_responses();
    if let Some(message) = my_towns_responses.sent_messages.last() {
        println!("My Towns menu: {}", message.text().unwrap_or("(no text)"));
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
}

/// Prints weather responses (current weather and forecast)
fn print_weather_responses(responses: &teloxide_tests::Responses, city: &str) {
    let total_messages = responses.sent_messages.len();
    
    println!("Bot responses after receiving '{}':", city);
    
    if total_messages >= 2 {
        // Get the last two messages (current weather and forecast)
        let current_weather_msg = &responses.sent_messages[total_messages - 2];
        let forecast_msg = &responses.sent_messages[total_messages - 1];
        
        println!("\n--- Current Weather Response ---");
        println!("{}", current_weather_msg.text().unwrap_or("(no text)"));
        
        println!("\n--- 7-Day Forecast Response ---");
        println!("{}", forecast_msg.text().unwrap_or("(no text)"));
    } else if total_messages >= 1 {
        // Only one message (might be an error or partial response)
        let last_msg = responses.sent_messages.last().unwrap();
        println!("Response: {}", last_msg.text().unwrap_or("(no text)"));
    } else {
        println!("No responses received");
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
