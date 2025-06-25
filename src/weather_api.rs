use reqwest;
use serde::Deserialize;
use std::env;
use teloxide::utils::markdown;

#[derive(Debug, Deserialize)]
pub struct CurrentWeather {
    pub location: Location,
    pub current: Current,
}

#[derive(Debug, Deserialize)]
pub struct Location {
    pub name: String,
    pub region: String,
    pub country: String,
}

#[derive(Debug, Deserialize)]
pub struct Current {
    #[serde(rename = "temp_c")]
    pub temperature: f32,
    #[serde(rename = "feelslike_c")]
    pub feels_like: f32,
    pub condition: Condition,
    #[serde(rename = "wind_kph")]
    pub wind_speed: f32,
    #[serde(rename = "wind_dir")]
    pub wind_direction: String,
    pub humidity: u32,
}

#[derive(Debug, Deserialize)]
pub struct Condition {
    pub text: String,
    pub icon: String,
}

#[derive(Debug, Deserialize)]
pub struct ForecastResponse {
    pub location: Location,
    pub forecast: Forecast,
}

#[derive(Debug, Deserialize)]
pub struct Forecast {
    #[serde(rename = "forecastday")]
    pub forecast_day: Vec<ForecastDay>,
}

#[derive(Debug, Deserialize)]
pub struct ForecastDay {
    pub date: String,
    pub day: Day,
}

#[derive(Debug, Deserialize)]
pub struct Day {
    #[serde(rename = "maxtemp_c")]
    pub max_temp: f32,
    #[serde(rename = "mintemp_c")]
    pub min_temp: f32,
    pub condition: Condition,
    #[serde(rename = "avghumidity")]
    pub avg_humidity: f32,
    #[serde(rename = "maxwind_kph")]
    pub max_wind: f32,
}

pub async fn get_current_weather(city: &str) -> Result<CurrentWeather, Box<dyn std::error::Error + Send + Sync>> {
    let api_key = env::var("WEATHER_API_KEY")
        .map_err(|_| "WEATHER_API_KEY environment variable not set")?;
    
    let url = format!(
        "http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no",
        api_key, city
    );

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;
    
    if !response.status().is_success() {
        return Err(format!("Weather API request failed with status: {}", response.status()).into());
    }

    let weather: CurrentWeather = response.json().await?;
    Ok(weather)
}

pub async fn get_forecast(city: &str, days: u8) -> Result<ForecastResponse, Box<dyn std::error::Error + Send + Sync>> {
    let api_key = env::var("WEATHER_API_KEY")
        .map_err(|_| "WEATHER_API_KEY environment variable not set")?;
    
    let url = format!(
        "http://api.weatherapi.com/v1/forecast.json?key={}&q={}&days={}&aqi=no&alerts=no",
        api_key, city, days
    );

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;
    
    if !response.status().is_success() {
        return Err(format!("Weather API request failed with status: {}", response.status()).into());
    }

    let forecast: ForecastResponse = response.json().await?;
    Ok(forecast)
}

pub fn format_current_weather(weather: &CurrentWeather) -> String {
    format!(
        "🌍 *{}*, {}, {}
🌡️ *Temperature:* {}°C \\(feels like {}°C\\)
☁️ *Condition:* {}
💨 *Wind:* {} km/h {}
💧 *Humidity:* {}%

\\-\\-\\-
Weather data provided by:
• [WeatherAPI\\.com](https://weatherapi.com)
• [OpenWeatherMap\\.org](https://openweathermap.org)
• [WeatherBit\\.io](https://weatherbit.io)",
        markdown::escape(&weather.location.name),
        markdown::escape(&weather.location.region),
        markdown::escape(&weather.location.country),
        markdown::escape(&weather.current.temperature.to_string()),
        markdown::escape(&weather.current.feels_like.to_string()),
        markdown::escape(&weather.current.condition.text),
        markdown::escape(&weather.current.wind_speed.to_string()),
        markdown::escape(&weather.current.wind_direction),
        weather.current.humidity
    )
}

pub fn format_forecast(forecast: &ForecastResponse) -> String {
    let mut message = format!(
        "📅 *7\\-Day Forecast for {}*, {}, {}\n\n",
        markdown::escape(&forecast.location.name),
        markdown::escape(&forecast.location.region),
        markdown::escape(&forecast.location.country)
    );

    for day in &forecast.forecast.forecast_day {
        message.push_str(&format!(
            "📆 *{}*\n🌡️ {}°C \\- {}°C \\| ☁️ {} \\| 💧 {}% \\| 💨 {} km/h\n\n",
            markdown::escape(&day.date),
            markdown::escape(&day.day.min_temp.to_string()),
            markdown::escape(&day.day.max_temp.to_string()),
            markdown::escape(&day.day.condition.text),
            markdown::escape(&day.day.avg_humidity.to_string()),
            markdown::escape(&day.day.max_wind.to_string())
        ));
    }

    message.push_str("\\-\\-\\-\nWeather data provided by:\n• [WeatherAPI\\.com](https://weatherapi.com)\n• [OpenWeatherMap\\.org](https://openweathermap.org)\n• [WeatherBit\\.io](https://weatherbit.io)");

    message
} 