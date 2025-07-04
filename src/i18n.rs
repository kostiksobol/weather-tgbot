use std::collections::HashMap;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Language {
    English,
    Russian,
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

impl Language {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "ru" | "russian" | "русский" => Language::Russian,
            _ => Language::English,
        }
    }
    
    pub fn to_str(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Russian => "ru",
        }
    }
    
    pub fn display_name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Russian => "Русский",
        }
    }
}

pub struct Translations {
    translations: HashMap<(&'static str, Language), &'static str>,
}

impl Translations {
    fn new() -> Self {
        let mut translations = HashMap::new();
        
        // Language selection
        translations.insert(("language_selection", Language::English), "Please select your language:");
        translations.insert(("language_selection", Language::Russian), "Пожалуйста, выберите ваш язык:");
        
        translations.insert(("language_changed", Language::English), "Language changed to English");
        translations.insert(("language_changed", Language::Russian), "Язык изменён на русский");
        
        // Main menu
        translations.insert(("welcome", Language::English), "Welcome! Please choose an option:");
        translations.insert(("welcome", Language::Russian), "Добро пожаловать! Пожалуйста, выберите опцию:");
        
        translations.insert(("weather", Language::English), "☀️ Weather");
        translations.insert(("weather", Language::Russian), "☀️ Погода");
        
        translations.insert(("forecast", Language::English), "📅 Forecast");
        translations.insert(("forecast", Language::Russian), "📅 Прогноз");
        
        translations.insert(("alerts", Language::English), "🚨 Alerts");
        translations.insert(("alerts", Language::Russian), "🚨 Оповещения");
        
        translations.insert(("interested_towns", Language::English), "🌍 Interested Towns");
        translations.insert(("interested_towns", Language::Russian), "🌍 Интересные города");
        
        translations.insert(("settings", Language::English), "⚙️ Settings");
        translations.insert(("settings", Language::Russian), "⚙️ Настройки");
        
        // Settings menu
        translations.insert(("settings_menu", Language::English), "Settings:");
        translations.insert(("settings_menu", Language::Russian), "Настройки:");
        
        translations.insert(("change_language", Language::English), "🌐 Change Language");
        translations.insert(("change_language", Language::Russian), "🌐 Изменить язык");
        
        translations.insert(("back_to_main", Language::English), "← Back to Main Menu");
        translations.insert(("back_to_main", Language::Russian), "← Назад в главное меню");
        
        // Weather menu
        translations.insert(("choose_weather_option", Language::English), "Choose current weather option:");
        translations.insert(("choose_weather_option", Language::Russian), "Выберите опцию текущей погоды:");
        
        translations.insert(("weather_for_city", Language::English), "🏙️ Weather for City");
        translations.insert(("weather_for_city", Language::Russian), "🏙️ Погода для города");
        
        translations.insert(("weather_home", Language::English), "🏠 Weather for Home Town");
        translations.insert(("weather_home", Language::Russian), "🏠 Погода для родного города");
        
        translations.insert(("enter_city_weather", Language::English), "Please enter the name of the city you want to get current weather for:");
        translations.insert(("enter_city_weather", Language::Russian), "Пожалуйста, введите название города для получения текущей погоды:");
        
        // Forecast menu
        translations.insert(("choose_forecast_option", Language::English), "Choose forecast option:");
        translations.insert(("choose_forecast_option", Language::Russian), "Выберите опцию прогноза:");
        
        translations.insert(("forecast_for_city", Language::English), "🏙️ Forecast for City");
        translations.insert(("forecast_for_city", Language::Russian), "🏙️ Прогноз для города");
        
        translations.insert(("forecast_home", Language::English), "🏠 Forecast for Home Town");
        translations.insert(("forecast_home", Language::Russian), "🏠 Прогноз для родного города");
        
        translations.insert(("enter_city_forecast", Language::English), "Please enter the name of the city you want to get forecast for:");
        translations.insert(("enter_city_forecast", Language::Russian), "Пожалуйста, введите название города для получения прогноза:");
        
        // Towns management
        translations.insert(("manage_towns", Language::English), "Manage your towns:");
        translations.insert(("manage_towns", Language::Russian), "Управление вашими городами:");
        
        translations.insert(("home_prefix", Language::English), "🏠 Home:");
        translations.insert(("home_prefix", Language::Russian), "🏠 Дом:");
        
        translations.insert(("view_weather", Language::English), "(View Weather)");
        translations.insert(("view_weather", Language::Russian), "(Посмотреть погоду)");
        
        translations.insert(("change_home_town", Language::English), "🔄 Change Home Town");
        translations.insert(("change_home_town", Language::Russian), "🔄 Изменить родной город");
        
        translations.insert(("set_home_town", Language::English), "🏠 Set Home Town");
        translations.insert(("set_home_town", Language::Russian), "🏠 Установить родной город");
        
        translations.insert(("interested_towns_separator", Language::English), "--- 🌍 Interested Towns ---");
        translations.insert(("interested_towns_separator", Language::Russian), "--- 🌍 Интересные города ---");
        
        translations.insert(("add_interested_town", Language::English), "➕ Add Interested Town");
        translations.insert(("add_interested_town", Language::Russian), "➕ Добавить интересный город");
        
        translations.insert(("remove_interested_town", Language::English), "🗑️ Remove Interested Town");
        translations.insert(("remove_interested_town", Language::Russian), "🗑️ Удалить интересный город");
        
        translations.insert(("enter_home_town", Language::English), "Please enter the name of your home town:");
        translations.insert(("enter_home_town", Language::Russian), "Пожалуйста, введите название вашего родного города:");
        
        translations.insert(("enter_interested_town", Language::English), "Please enter the name of the town you're interested in:");
        translations.insert(("enter_interested_town", Language::Russian), "Пожалуйста, введите название города, который вас интересует:");
        
        translations.insert(("no_home_town", Language::English), "You haven't set a home town yet. Please enter your home town name:");
        translations.insert(("no_home_town", Language::Russian), "Вы ещё не установили родной город. Пожалуйста, введите название вашего родного города:");
        
        translations.insert(("home_town_set", Language::English), "Home town set to:");
        translations.insert(("home_town_set", Language::Russian), "Родной город установлен:");
        
        translations.insert(("town_added", Language::English), "Added to your interested towns:");
        translations.insert(("town_added", Language::Russian), "Добавлен в ваши интересные города:");
        
        translations.insert(("town_removed", Language::English), "Removed from your interested towns");
        translations.insert(("town_removed", Language::Russian), "Удалён из ваших интересных городов");
        
        translations.insert(("no_towns_to_remove", Language::English), "You don't have any interested towns to remove.");
        translations.insert(("no_towns_to_remove", Language::Russian), "У вас нет интересных городов для удаления.");
        
        translations.insert(("select_town_to_remove", Language::English), "Select a town to remove:");
        translations.insert(("select_town_to_remove", Language::Russian), "Выберите город для удаления:");
        
        // Alerts
        translations.insert(("alerts_management", Language::English), "🚨 Weather Alerts Management\n\nChoose an option:");
        translations.insert(("alerts_management", Language::Russian), "🚨 Управление погодными оповещениями\n\nВыберите опцию:");
        
        translations.insert(("add_alert", Language::English), "➕ Add Alert");
        translations.insert(("add_alert", Language::Russian), "➕ Добавить оповещение");
        
        translations.insert(("remove_alert", Language::English), "🗑️ Remove Alert");
        translations.insert(("remove_alert", Language::Russian), "🗑️ Удалить оповещение");
        
        translations.insert(("no_alerts", Language::English), "You don't have any alerts to remove.");
        translations.insert(("no_alerts", Language::Russian), "У вас нет оповещений для удаления.");
        
        translations.insert(("choose_alert_type", Language::English), "Choose alert type:");
        translations.insert(("choose_alert_type", Language::Russian), "Выберите тип оповещения:");
        
        translations.insert(("standard_alert", Language::English), "🚨 Standard Weather Alert");
        translations.insert(("standard_alert", Language::Russian), "🚨 Стандартное погодное оповещение");
        
        translations.insert(("temperature_alert", Language::English), "🌡️ Temperature Alert");
        translations.insert(("temperature_alert", Language::Russian), "🌡️ Температурное оповещение");
        
        translations.insert(("wind_alert", Language::English), "💨 Wind Speed Alert");
        translations.insert(("wind_alert", Language::Russian), "💨 Оповещение о скорости ветра");
        
        translations.insert(("humidity_alert", Language::English), "💧 Humidity Alert");
        translations.insert(("humidity_alert", Language::Russian), "💧 Оповещение о влажности");
        
        translations.insert(("alert_created", Language::English), "✅ Weather alert created for");
        translations.insert(("alert_created", Language::Russian), "✅ Погодное оповещение создано для");
        
        translations.insert(("with_hours_warning", Language::English), "with {} hours advance warning!");
        translations.insert(("with_hours_warning", Language::Russian), "с предупреждением за {} часов!");
        
        // Common
        translations.insert(("cancel", Language::English), "❌ Cancel");
        translations.insert(("cancel", Language::Russian), "❌ Отмена");
        
        translations.insert(("operation_cancelled", Language::English), "Operation cancelled. Choose another option:");
        translations.insert(("operation_cancelled", Language::Russian), "Операция отменена. Выберите другую опцию:");
        
        translations.insert(("choose_another_option", Language::English), "Choose another option:");
        translations.insert(("choose_another_option", Language::Russian), "Выберите другую опцию:");
        
        translations.insert(("error_weather", Language::English), "Sorry, I couldn't get the weather for");
        translations.insert(("error_weather", Language::Russian), "Извините, я не смог получить погоду для");
        
        translations.insert(("error_forecast", Language::English), "Sorry, I couldn't get the forecast for");
        translations.insert(("error_forecast", Language::Russian), "Извините, я не смог получить прогноз для");
        
        translations.insert(("error", Language::English), "Error:");
        translations.insert(("error", Language::Russian), "Ошибка:");
        
        translations.insert(("your_home_town", Language::English), "your home town");
        translations.insert(("your_home_town", Language::Russian), "вашего родного города");
        
        // Alert messages
        translations.insert(("enter_city_standard_alert", Language::English), "Enter the city name for standard weather alerts:");
        translations.insert(("enter_city_standard_alert", Language::Russian), "Введите название города для стандартных погодных оповещений:");
        
        translations.insert(("enter_city_temperature_alert", Language::English), "Enter the city name for temperature alerts:");
        translations.insert(("enter_city_temperature_alert", Language::Russian), "Введите название города для температурных оповещений:");
        
        translations.insert(("enter_city_wind_alert", Language::English), "Enter the city name for wind speed alerts:");
        translations.insert(("enter_city_wind_alert", Language::Russian), "Введите название города для оповещений о скорости ветра:");
        
        translations.insert(("enter_city_humidity_alert", Language::English), "Enter the city name for humidity alerts:");
        translations.insert(("enter_city_humidity_alert", Language::Russian), "Введите название города для оповещений о влажности:");
        
        translations.insert(("how_many_hours", Language::English), "🕐 How many hours ahead should I warn you about weather in '{}'?\n\nEnter a number (1-72 hours):");
        translations.insert(("how_many_hours", Language::Russian), "🕐 За сколько часов предупреждать о погоде в '{}'?\n\nВведите число (1-72 часа):");
        
        translations.insert(("enter_min_temp", Language::English), "Enter minimum temperature threshold (°C) or type 'skip' to skip:");
        translations.insert(("enter_min_temp", Language::Russian), "Введите минимальный порог температуры (°C) или введите 'skip' для пропуска:");
        
        translations.insert(("enter_max_temp", Language::English), "Enter maximum temperature threshold (°C) or type 'skip' to skip:");
        translations.insert(("enter_max_temp", Language::Russian), "Введите максимальный порог температуры (°C) или введите 'skip' для пропуска:");
        
        translations.insert(("enter_max_wind", Language::English), "Enter maximum wind speed threshold (km/h):");
        translations.insert(("enter_max_wind", Language::Russian), "Введите максимальный порог скорости ветра (км/ч):");
        
        translations.insert(("enter_min_humidity", Language::English), "Enter minimum humidity threshold (%) or type 'skip' to skip:");
        translations.insert(("enter_min_humidity", Language::Russian), "Введите минимальный порог влажности (%) или введите 'skip' для пропуска:");
        
        translations.insert(("enter_max_humidity", Language::English), "Enter maximum humidity threshold (%) or type 'skip' to skip:");
        translations.insert(("enter_max_humidity", Language::Russian), "Введите максимальный порог влажности (%) или введите 'skip' для пропуска:");
        
        translations.insert(("hours_temp_warning", Language::English), "🕐 How many hours ahead should I warn you about temperature changes?\n\nEnter a number (1-72 hours):");
        translations.insert(("hours_temp_warning", Language::Russian), "🕐 За сколько часов предупреждать об изменениях температуры?\n\nВведите число (1-72 часа):");
        
        translations.insert(("hours_wind_warning", Language::English), "🕐 How many hours ahead should I warn you about wind conditions?\n\nEnter a number (1-72 hours):");
        translations.insert(("hours_wind_warning", Language::Russian), "🕐 За сколько часов предупреждать о ветровых условиях?\n\nВведите число (1-72 часа):");
        
        translations.insert(("hours_humidity_warning", Language::English), "🕐 How many hours ahead should I warn you about humidity changes?\n\nEnter a number (1-72 hours):");
        translations.insert(("hours_humidity_warning", Language::Russian), "🕐 За сколько часов предупреждать об изменениях влажности?\n\nВведите число (1-72 часа):");
        
        // Message handler responses
        translations.insert(("town_already_exists", Language::English), "This town is already in your interested towns list");
        translations.insert(("town_already_exists", Language::Russian), "Этот город уже есть в вашем списке интересных городов");
        
        translations.insert(("weather_error_check_city", Language::English), "Sorry, I couldn't get the weather for '{}'. Please check the city name and try again.\n\nError: {}");
        translations.insert(("weather_error_check_city", Language::Russian), "Извините, я не смог получить погоду для '{}'. Пожалуйста, проверьте название города и попробуйте снова.\n\nОшибка: {}");
        
        translations.insert(("invalid_temp", Language::English), "Invalid temperature value. Please enter a valid number or 'skip':");
        translations.insert(("invalid_temp", Language::Russian), "Неверное значение температуры. Пожалуйста, введите правильное число или 'skip':");
        
        translations.insert(("invalid_wind", Language::English), "Invalid wind speed value. Please enter a valid number:");
        translations.insert(("invalid_wind", Language::Russian), "Неверное значение скорости ветра. Пожалуйста, введите правильное число:");
        
        translations.insert(("invalid_humidity", Language::English), "Invalid humidity value. Please enter a valid number (0-100) or 'skip':");
        translations.insert(("invalid_humidity", Language::Russian), "Неверное значение влажности. Пожалуйста, введите правильное число (0-100) или 'skip':");
        
        translations.insert(("invalid_hours", Language::English), "⚠️ Please enter a number between 1 and 72 hours:");
        translations.insert(("invalid_hours", Language::Russian), "⚠️ Пожалуйста, введите число от 1 до 72 часов:");
        
        translations.insert(("invalid_number", Language::English), "⚠️ Please enter a valid number:");
        translations.insert(("invalid_number", Language::Russian), "⚠️ Пожалуйста, введите правильное число:");
        
        translations.insert(("error_no_pending_alert", Language::English), "Error: No pending alert data found.");
        translations.insert(("error_no_pending_alert", Language::Russian), "Ошибка: Данные ожидающего оповещения не найдены.");
        
        translations.insert(("back_to_alerts", Language::English), "← Back to Alerts Menu");
        translations.insert(("back_to_alerts", Language::Russian), "← Назад к меню оповещений");
        
        translations.insert(("select_alert_to_remove", Language::English), "Select an alert to remove:");
        translations.insert(("select_alert_to_remove", Language::Russian), "Выберите оповещение для удаления:");
        
        translations.insert(("alert_removed", Language::English), "Alert removed successfully!");
        translations.insert(("alert_removed", Language::Russian), "Оповещение успешно удалено!");
        
        translations.insert(("unknown_button", Language::English), "Unknown button.");
        translations.insert(("unknown_button", Language::Russian), "Неизвестная кнопка.");
        
        translations.insert(("error_unknown_alert", Language::English), "Error: Unknown alert type. Please try again.");
        translations.insert(("error_unknown_alert", Language::Russian), "Ошибка: Неизвестный тип оповещения. Попробуйте снова.");
        
        translations.insert(("no_home_town_use_set", Language::English), "You haven't set a home town yet. Use 'Set Home Town' to set one.");
        translations.insert(("no_home_town_use_set", Language::Russian), "Вы ещё не установили родной город. Используйте 'Установить родной город' для установки.");
        
        translations.insert(("select_town_separator", Language::English), "--- 🗑️ Select Town to Remove ---");
        translations.insert(("select_town_separator", Language::Russian), "--- 🗑️ Выберите город для удаления ---");
        
        translations.insert(("back_to_interested_towns", Language::English), "← Back to Interested Towns");
        translations.insert(("back_to_interested_towns", Language::Russian), "← Назад к интересным городам");
        
        translations.insert(("your_alerts_separator", Language::English), "--- 🚨 Your Weather Alerts ---");
        translations.insert(("your_alerts_separator", Language::Russian), "--- 🚨 Ваши погодные оповещения ---");
        
        translations.insert(("select_alert_separator", Language::English), "--- 🗑️ Select Alert to Remove ---");
        translations.insert(("select_alert_separator", Language::Russian), "--- 🗑️ Выберите оповещение для удаления ---");
        
        // Alert types in buttons
        translations.insert(("alert_type_standard", Language::English), "Standard");
        translations.insert(("alert_type_standard", Language::Russian), "Стандартное");
        
        translations.insert(("alert_type_temperature", Language::English), "Temperature");
        translations.insert(("alert_type_temperature", Language::Russian), "Температура");
        
        translations.insert(("alert_type_wind", Language::English), "Wind");
        translations.insert(("alert_type_wind", Language::Russian), "Ветер");
        
        translations.insert(("alert_type_humidity", Language::English), "Humidity");
        translations.insert(("alert_type_humidity", Language::Russian), "Влажность");
        
        translations.insert(("active", Language::English), "Active");
        translations.insert(("active", Language::Russian), "Активно");
        
        translations.insert(("inactive", Language::English), "Inactive");
        translations.insert(("inactive", Language::Russian), "Неактивно");
        
        // Alert check messages
        translations.insert(("alert_triggered", Language::English), "ALERT TRIGGERED!");
        translations.insert(("alert_triggered", Language::Russian), "ОПОВЕЩЕНИЕ СРАБОТАЛО!");
        
        translations.insert(("all_good", Language::English), "All Good");
        translations.insert(("all_good", Language::Russian), "Всё хорошо");
        
        translations.insert(("city", Language::English), "City:");
        translations.insert(("city", Language::Russian), "Город:");
        
        translations.insert(("description", Language::English), "Description:");
        translations.insert(("description", Language::Russian), "Описание:");
        
        translations.insert(("current_weather", Language::English), "Current Weather:");
        translations.insert(("current_weather", Language::Russian), "Текущая погода:");
        
        translations.insert(("temperature", Language::English), "Temperature:");
        translations.insert(("temperature", Language::Russian), "Температура:");
        
        translations.insert(("condition", Language::English), "Condition:");
        translations.insert(("condition", Language::Russian), "Состояние:");
        
        translations.insert(("wind", Language::English), "Wind:");
        translations.insert(("wind", Language::Russian), "Ветер:");
        
        translations.insert(("humidity", Language::English), "Humidity:");
        translations.insert(("humidity", Language::Russian), "Влажность:");
        
        translations.insert(("created", Language::English), "Created:");
        translations.insert(("created", Language::Russian), "Создано:");
        
        translations.insert(("last_triggered", Language::English), "Last triggered:");
        translations.insert(("last_triggered", Language::Russian), "Последнее срабатывание:");
        
        translations.insert(("never_triggered", Language::English), "Never triggered");
        translations.insert(("never_triggered", Language::Russian), "Никогда не срабатывало");
        
        translations.insert(("failed_weather_data", Language::English), "❌ Failed to get weather data for {}: {}");
        translations.insert(("failed_weather_data", Language::Russian), "❌ Не удалось получить данные о погоде для {}: {}");
        
        translations.insert(("error_checking_alert", Language::English), "❌ Error checking alert: {}");
        translations.insert(("error_checking_alert", Language::Russian), "❌ Ошибка проверки оповещения: {}");
        
        translations.insert(("alert_not_found", Language::English), "❌ Alert not found.");
        translations.insert(("alert_not_found", Language::Russian), "❌ Оповещение не найдено.");
        
        translations.insert(("standard_weather_alert", Language::English), "🚨 Standard Weather Alert");
        translations.insert(("standard_weather_alert", Language::Russian), "🚨 Стандартное погодное оповещение");
        
        translations.insert(("temperature_alert_desc", Language::English), "🌡️ Temperature Alert");
        translations.insert(("temperature_alert_desc", Language::Russian), "🌡️ Температурное оповещение");
        
        translations.insert(("wind_speed_alert_desc", Language::English), "💨 Wind Speed Alert (max {} km/h)");
        translations.insert(("wind_speed_alert_desc", Language::Russian), "💨 Оповещение о скорости ветра (макс {} км/ч)");
        
        translations.insert(("humidity_alert_desc", Language::English), "💧 Humidity Alert");
        translations.insert(("humidity_alert_desc", Language::Russian), "💧 Оповещение о влажности");
        
        translations.insert(("min", Language::English), "min");
        translations.insert(("min", Language::Russian), "мин");
        
        translations.insert(("max", Language::English), "max");
        translations.insert(("max", Language::Russian), "макс");
        
        translations.insert(("kmh", Language::English), "km/h");
        translations.insert(("kmh", Language::Russian), "км/ч");
        
        translations.insert(("error_forecast_hometown", Language::English), "Sorry, I couldn't get the forecast for your home town '{}'. Error: {}");
        translations.insert(("error_forecast_hometown", Language::Russian), "Извините, не удалось получить прогноз для вашего родного города '{}'. Ошибка: {}");
        
        translations.insert(("error_weather_hometown", Language::English), "Sorry, I couldn't get the weather for your home town '{}'. Error: {}");
        translations.insert(("error_weather_hometown", Language::Russian), "Извините, не удалось получить погоду для вашего родного города '{}'. Ошибка: {}");

        // format_current_weather
        translations.insert(("temperature_label", Language::English), "Temperature:");
        translations.insert(("temperature_label", Language::Russian), "Температура:");
        translations.insert(("feels_like_label", Language::English), "feels like");
        translations.insert(("feels_like_label", Language::Russian), "ощущается как");
        translations.insert(("condition_label", Language::English), "Condition:");
        translations.insert(("condition_label", Language::Russian), "Состояние:");
        translations.insert(("wind_label", Language::English), "Wind:");
        translations.insert(("wind_label", Language::Russian), "Ветер:");
        translations.insert(("humidity_label", Language::English), "Humidity:");
        translations.insert(("humidity_label", Language::Russian), "Влажность:");
        translations.insert(("more_info_label", Language::English), "More weather info:");
        translations.insert(("more_info_label", Language::Russian), "Больше информации о погоде:");

        // format_forecast
        translations.insert(("forecast_title", Language::English), "3-Day Forecast for");
        translations.insert(("forecast_title", Language::Russian), "Прогноз на 3 дня для");

        // Alert Descriptions
        translations.insert(("alert_desc_standard", Language::English), "Standard weather warnings for {} (for {} hours)");
        translations.insert(("alert_desc_standard", Language::Russian), "Стандартные предупреждения о погоде для {} (за {} ч.)");
        translations.insert(("alert_desc_temp_range", Language::English), "Temperature outside the range {}°C - {}°C in {} (for {} hours)");
        translations.insert(("alert_desc_temp_range", Language::Russian), "Температура вне диапазона {}°C - {}°C в {} (за {} ч.)");
        translations.insert(("alert_desc_temp_min", Language::English), "Temperature below {}°C in {} (for {} hours)");
        translations.insert(("alert_desc_temp_min", Language::Russian), "Температура ниже {}°C в {} (за {} ч.)");
        translations.insert(("alert_desc_temp_max", Language::English), "Temperature above {}°C in {} (for {} hours)");
        translations.insert(("alert_desc_temp_max", Language::Russian), "Температура выше {}°C в {} (за {} ч.)");
        translations.insert(("alert_desc_temp_none", Language::English), "Temperature monitoring in {} (for {} hours)");
        translations.insert(("alert_desc_temp_none", Language::Russian), "Контроль температуры в {} (за {} ч.)");
        translations.insert(("alert_desc_wind", Language::English), "Wind speed above {} km/h in {} (for {} hours)");
        translations.insert(("alert_desc_wind", Language::Russian), "Скорость ветра выше {} км/ч в {} (за {} ч.)");
        translations.insert(("alert_desc_humidity_range", Language::English), "Humidity outside the range {}% - {}% in {} (for {} hours)");
        translations.insert(("alert_desc_humidity_range", Language::Russian), "Влажность вне диапазона {}% - {}% в {} (за {} ч.)");
        translations.insert(("alert_desc_humidity_min", Language::English), "Humidity below {}% in {} (for {} hours)");
        translations.insert(("alert_desc_humidity_min", Language::Russian), "Влажность ниже {}% в {} (за {} ч.)");
        translations.insert(("alert_desc_humidity_max", Language::English), "Humidity above {}% in {} (for {} hours)");
        translations.insert(("alert_desc_humidity_max", Language::Russian), "Влажность выше {}% в {} (за {} ч.)");
        translations.insert(("alert_desc_humidity_none", Language::English), "Humidity monitoring in {} (for {} hours)");
        translations.insert(("alert_desc_humidity_none", Language::Russian), "Контроль влажности в {} (за {} ч.)");

        Self { translations }
    }
    
    pub fn get(&self, key: &str, lang: Language) -> &'static str {
        self.translations
            .get(&(key, lang))
            .copied()
            .unwrap_or_else(|| {
                log::warn!("Missing translation for key '{}' in language {:?}", key, lang);
                self.translations
                    .get(&(key, Language::English))
                    .copied()
                    .unwrap_or("[MISSING TRANSLATION]")
            })
    }
}

pub static TRANSLATIONS: Lazy<Translations> = Lazy::new(Translations::new);

pub fn t(key: &str, lang: Language) -> &'static str {
    TRANSLATIONS.get(key, lang)
}

pub fn t_condition(condition: &str, lang: Language) -> &str {
    match lang {
        Language::Russian => match condition.to_lowercase().as_str() {
            "sunny" => "Солнечно",
            "clear" => "Ясно",
            "partly cloudy" => "Переменная облачность",
            "cloudy" => "Облачно",
            "overcast" => "Пасмурно",
            "mist" => "Туман",
            "patchy rain possible" => "Возможен небольшой дождь",
            "patchy rain nearby" => "Местами небольшой дождь",
            "patchy snow possible" => "Возможен небольшой снег",
            "patchy sleet possible" => "Возможен небольшой мокрый снег",
            "patchy freezing drizzle possible" => "Возможна небольшая изморозь",
            "thundery outbreaks possible" => "Возможны грозы",
            "blowing snow" => "Метель",
            "blizzard" => "Буран",
            "fog" => "Туман",
            "freezing fog" => "Ледяной туман",
            "patchy light drizzle" => "Местами слабая морось",
            "light drizzle" => "Слабая морось",
            "freezing drizzle" => "Изморозь",
            "heavy freezing drizzle" => "Сильная изморозь",
            "patchy light rain" => "Местами небольшой дождь",
            "light rain" => "Небольшой дождь",
            "moderate rain at times" => "Временами умеренный дождь",
            "moderate rain" => "Умеренный дождь",
            "heavy rain at times" => "Временами сильный дождь",
            "heavy rain" => "Сильный дождь",
            "light freezing rain" => "Слабый ледяной дождь",
            "moderate or heavy freezing rain" => "Умеренный или сильный ледяной дождь",
            "light sleet" => "Слабый мокрый снег",
            "moderate or heavy sleet" => "Умеренный или сильный мокрый снег",
            "patchy light snow" => "Местами небольшой снег",
            "light snow" => "Небольшой снег",
            "patchy moderate snow" => "Местами умеренный снег",
            "moderate snow" => "Умеренный снег",
            "patchy heavy snow" => "Местами сильный снег",
            "heavy snow" => "Сильный снег",
            "ice pellets" => "Ледяная крупа",
            "light rain shower" => "Небольшой ливень",
            "moderate or heavy rain shower" => "Умеренный или сильный ливень",
            "torrential rain shower" => "Проливной ливень",
            "light sleet showers" => "Небольшой ливень с мокрым снегом",
            "moderate or heavy sleet showers" => "Умеренный или сильный ливень с мокрым снегом",
            "light snow showers" => "Небольшой снегопад",
            "moderate or heavy snow showers" => "Умеренный или сильный снегопад",
            "light showers of ice pellets" => "Небольшой ливень с ледяной крупой",
            "moderate or heavy showers of ice pellets" => "Умеренный или сильный ливень с ледяной крупой",
            "patchy light rain with thunder" => "Местами небольшой дождь с грозой",
            "moderate or heavy rain with thunder" => "Умеренный или сильный дождь с грозой",
            "patchy light snow with thunder" => "Местами небольшой снег с грозой",
            "moderate or heavy snow with thunder" => "Умеренный или сильный снег с грозой",
            _ => condition,
        },
        Language::English => condition,
    }
} 