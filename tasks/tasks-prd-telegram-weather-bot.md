## Relevant Files

- `src/main.rs` - Main application entry point, bot initialization, and the main update loop.
- `src/lib.rs` - Library root that exports the handler_tree function and bot module.
- `src/bot/mod.rs` - Contains all the bot logic including Command enum, answer handler, and keyboard creation.
- `src/bot/handlers.rs` - Contains all the `teloxide` handlers for commands (`/start`) and callback queries from inline buttons.
- `src/bot/keyboards.rs` - Functions dedicated to creating and managing the various inline keyboard layouts (main menu, town lists, etc.).
- `src/state.rs` - Defines the in-memory data structures (`UserData`) and the shared state management (`Arc<Mutex<HashMap<...>>>`) for session data.
- `src/weather_api.rs` - Module responsible for all communication with `weatherapi.com`, including data fetching and parsing.
- `src/alerts.rs` - Contains the logic for the background polling task that checks for and sends weather alerts to users.
- `tester/src/main.rs` - A separate crate for simulating user-bot conversations. Includes comprehensive weather conversation simulation testing.
- `.env` - Stores secrets like the Telegram bot token and the weather API key.
- `Cargo.toml` - Manages all Rust project dependencies.

### Notes

- The `tester` crate uses the `teloxide-tests` library to mock bot interactions.
- `teloxide` is pinned to version `0.15.0` to maintain compatibility with `teloxide-tests`.
- The output from the `tester` shows a strict script of user commands and bot responses, including button interactions.
- Use `cargo run -p tester` to run the conversation simulation.

## Tasks

- [x] 1.0 Setup Core Bot Infrastructure
  - [x] 1.1 Set up a new Rust project using Cargo and add essential dependencies: `teloxide`, `tokio`, `dotenv`, `reqwest`, `serde`, `serde_json`, and `teloxide-tests` for the tester crate.
  - [x] 1.2 Create a `.env` file to manage environment variables for the Telegram bot token and weather API key.
  - [x] 1.3 Implement the main application loop and a basic command handler for `/start` that sends a welcome message.
  - [x] 1.4 Implement a main menu with inline keyboard buttons for "Get Weather," "My Towns," and "Alerts."
  - [x] 1.5 [Testing] Create the initial conversation simulation in the `tester` crate for the `/start` command and button interactions using the `teloxide-tests` library.
- [x] 2.0 Implement Core Weather Lookups
  - [x] 2.1 Create the `src/weather_api.rs` module to handle all interactions with the weather API.
  - [x] 2.2 Implement a function to fetch and parse the current weather for a city, including temperature, "feels like," condition, wind, and humidity.
  - [x] 2.3 Implement a function to fetch and parse the 7-day forecast for a city.
  - [x] 2.4 Create a handler that responds to the "Get Weather" button, prompts for a city, and displays the formatted weather/forecast data.
  - [x] 2.5 Ensure the weather report messages include the required links to `weatherapi.com`, `openweathermap.org`, and `weatherbit.io`.
  - [x] 2.6 [Testing] Add conversation simulation for getting current weather and a 7-day forecast.
- [x] 3.0 Implement User Personalization (Home & Interested Towns)
  - [x] 3.1 In `src/state.rs`, define a `UserData` struct to store `home_town` and `interested_towns`.
  - [x] 3.2 Implement a shared state solution (e.g., `Arc<Mutex<HashMap<...>>>`) to store `UserData` in memory, keyed by `chat_id`.
  - [x] 3.3 Create handlers for the "My Towns" menu: "Set Home," "Add Interested Town," "View Interested Towns," etc.
  - [x] 3.4 Implement the logic for setting, changing, and displaying the home town.
  - [x] 3.5 Implement logic for adding and removing interested towns.
  - [x] 3.6 Implement the dynamic keyboard generation for the list of interested towns. Pressing a town button should fetch its current weather.
  - [x] 3.7 [Testing] Add conversation simulation for setting a home town, adding interested towns, and getting weather for them.
- [ ] 4.0 Implement Weather Alert System
  - [ ] 4.1 Define data structures for alert subscriptions (`AlertSubscription`) to be stored in the `UserData` struct.
  - [ ] 4.2 Implement the UI flow (menus and buttons) for subscribing to default alerts and creating custom alerts for a city.
  - [ ] 4.3 Implement handlers for viewing and unsubscribing from active alerts.
  - [ ] 4.4 In `src/alerts.rs`, create a separate, asynchronous task that runs every 5 minutes.
  - [ ] 4.5 The background task will iterate through all user subscriptions, fetch weather, and check if alert conditions are met.
  - [ ] 4.6 If an alert is triggered, the task will send a message to the corresponding user's `chat_id`.
  - [ ] 4.7 [Testing] Add conversation simulation for creating, viewing, and deleting alerts. Note: Testing the background polling directly is out of scope for the simulation.
- [ ] 5.0 Finalize Conversation Simulation Crate
  - [ ] 5.1 Create the new binary crate `tester` within the workspace (`cargo new --bin tester`) and add `teloxide-tests` as a dependency.
  - [ ] 5.2 In `tester/src/main.rs`, use the test environment provided by `teloxide-tests` to set up a mock bot and user.
  - [ ] 5.3 Define helper functions that use the library to simulate user actions (e.g., sending text, pressing buttons) and print the bot's replies to the console.
  - [ ] 5.4 Write a `main` function that uses these helpers to simulate a full user journey, printing the "conversation" to the console. 