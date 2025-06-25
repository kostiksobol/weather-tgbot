# Product Requirements Document: Telegram Weather Bot

## 1. Introduction/Overview

This document outlines the requirements for a Telegram bot that provides users with quick, on-the-go weather information. The bot will be written in Rust and will feature a highly interactive, button-driven interface to minimize user text input. The core purpose is to deliver fast, accurate, and personalized weather data directly within a user's Telegram chat. All user data will be session-based, meaning it is tied to a specific chat and is not carried over if the user starts a new chat.

## 2. Goals

*   **Speed & Accessibility:** Provide users with immediate access to current and forecasted weather conditions.
*   **User Experience:** Create a seamless and intuitive user experience primarily through inline keyboard buttons rather than text commands.
*   **Personalization:** Allow users to save a "home town" and a list of "interested towns" for quick access.
*   **Proactive Alerts:** Deliver timely, automated weather alerts based on both default server conditions and user-defined criteria.

## 3. User Stories

*   **As a user,** I want to get the current weather for any city by selecting an option and typing the city's name.
*   **As a user,** I want to get a 7-day weather forecast for any city.
*   **As a user,** I want to set a "home town" so I can get its weather with a single button press.
*   **As a user,** I want to maintain a list of "interested towns" and see them as buttons, so I can tap one to get its current weather instantly.
*   **As a user,** I want to subscribe to default weather alerts (e.g., hurricanes, heat waves) for a specific city.
*   **As a user,** I want to create custom alerts for a city based on my own conditions (e.g., notify me if the temperature drops below 10°C or if it's going to rain).
*   **As a user,** I want to see a list of all my active alert subscriptions.
*   **As a user,** I want to unsubscribe from an alert at any time.
*   **As a user,** I expect all my settings (home town, subscriptions) to be forgotten if I delete the chat and start a new one.

## 4. Functional Requirements

### Core Weather Functions
1.  **Get Current Weather:** The user can request the current weather for any city.
2.  **Get 7-Day Forecast:** The user can request a 7-day weather forecast for any city.
3.  **Weather Data Display:** All weather reports must include:
    *   Temperature (°C/°F)
    *   "Feels like" temperature
    *   Weather condition (e.g., Sunny, Cloudy, Rain)
    *   Wind speed
    *   Humidity
    *   An inline link to `weatherapi.com`, `openweathermap.org`, and `weatherbit.io` for more details.

### Personalization
4.  **Home Town:**
    *   The user must be able to set, change, and remove one city as their "home town".
    *   Dedicated buttons must be available to get the current weather and 7-day forecast for the home town.
5.  **Interested Towns:**
    *   The user must be able to add and remove cities from a list of "interested towns".
    *   The bot must display this list to the user as a set of inline keyboard buttons.
    *   When the user presses a button for an interested town, the bot will **only** display the **current weather** for that town.

### Weather Alerts
6.  **Default Alerts:** The user can subscribe to and unsubscribe from default severe weather alerts provided by the weather API for any city.
7.  **Custom Alerts:**
    *   The user can create custom alerts for a city based on specific conditions (e.g., temperature, rain, heavy rain).
    *   This functionality must be managed through an interactive menu system.
8.  **View Subscriptions:** The user must be able to view a list of all their active alert subscriptions.
9.  **Alert Polling:** The bot backend must check for alert conditions for all active subscriptions every **5 minutes**.

### System
10. **Interaction Model:** All user interactions must be handled through inline keyboard buttons where possible. Text input should only be required for specifying city names.
11. **Data Persistence:** All user-specific data (home town, interested towns, subscriptions) is tied to the Telegram `chat_id`. This data is ephemeral and should be cleared if the user starts a new chat.

## 5. Non-Goals (Out of Scope)

*   Historical weather data lookup.
*   Graphical weather maps.
*   User accounts that persist across different chats or devices.
*   Multi-language support (English only for the first version).

## 6. Design Considerations

*   **UI:** The interface should be clean, modern, and easy to navigate using Telegram's inline keyboards. The goal is to make the bot feel like a mini-app within Telegram.
*   **Commands:** Use a main menu with clear buttons like "Get Weather", "My Towns", and "Alerts" to guide the user.
*   **Feedback:** The bot should provide clear confirmation messages for actions like setting a home town or subscribing to an alert.

## 7. Technical Considerations

*   **Language:** The bot will be developed in **Rust**.
*   **Primary API:** The main source for weather data is `weatherapi.com`.
*   **API Failure:** If `weatherapi.com` is unavailable, the bot should notify the user that the service is temporarily down. It should **not** attempt to failover to another API.
*   **API Polling Rate:** The 5-minute polling interval for alerts for every subscription will generate a significant number of API calls. The implementation must be robust enough to handle this and manage API rate limits effectively.
*   **Data Storage:** A persistent database is not required. User data can be managed in memory or a simple key-value store (like Redis) with keys based on `chat_id`.

## 8. Success Metrics

*   To be defined at a later stage. The initial focus is on delivering a stable and functional bot.

## 9. Open Questions

*   What is the definitive list of default alert types we will support from the weather API (e.g., "Tornado", "Flood", "High Wind")?
*   What is the initial set of conditions a user can select for custom alerts (e.g., Temperature (above/below), Wind Speed (above), Condition (is/is not))? 