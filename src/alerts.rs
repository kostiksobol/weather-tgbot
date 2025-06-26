use crate::state::{WeatherAlert, AlertType};
use crate::weather_api::{CurrentWeather, get_current_weather};
use uuid::Uuid;

pub struct AlertChecker;

impl AlertChecker {
    pub async fn check_alert(alert: &WeatherAlert) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let weather = get_current_weather(&alert.city).await?;
        
        match &alert.alert_type {
            AlertType::StandardWeatherAlert => {
                // Проверяем на наличие экстремальных погодных условий
                Self::check_standard_weather_conditions(&weather)
            }
            AlertType::TemperatureThreshold { min, max } => {
                let temp = weather.current.temperature;
                let triggered = if let Some(min_temp) = min {
                    temp < *min_temp
                } else {
                    false
                } || if let Some(max_temp) = max {
                    temp > *max_temp
                } else {
                    false
                };
                Ok(triggered)
            }
            AlertType::WindSpeed { max } => {
                Ok(weather.current.wind_speed > *max)
            }
            AlertType::Humidity { min, max } => {
                let humidity = weather.current.humidity;
                let triggered = if let Some(min_hum) = min {
                    humidity < *min_hum
                } else {
                    false
                } || if let Some(max_hum) = max {
                    humidity > *max_hum
                } else {
                    false
                };
                Ok(triggered)
            }
        }
    }
    
    fn check_standard_weather_conditions(weather: &CurrentWeather) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let condition_text = weather.current.condition.text.to_lowercase();
        let temp = weather.current.temperature;
        let wind_speed = weather.current.wind_speed;
        
        // Список ключевых слов для экстремальных погодных условий
        let extreme_conditions = [
            "thunderstorm", "storm", "tornado", "hurricane", "cyclone",
            "blizzard", "hail", "snow", "ice", "freeze", "freezing",
            "extreme", "severe", "heavy", "violent", "dangerous"
        ];
        
        // Проверяем условия
        let extreme_weather = extreme_conditions.iter()
            .any(|&condition| condition_text.contains(condition));
        
        let extreme_temperature = temp > 40.0 || temp < -20.0;
        let extreme_wind = wind_speed > 50.0;
        
        Ok(extreme_weather || extreme_temperature || extreme_wind)
    }
    
    pub fn format_alert_message(alert: &WeatherAlert, weather: &CurrentWeather) -> String {
        let alert_type_str = match &alert.alert_type {
            AlertType::StandardWeatherAlert => "🚨 Экстремальные погодные условия",
            AlertType::TemperatureThreshold { .. } => "🌡️ Превышение температурного порога",
            AlertType::WindSpeed { .. } => "💨 Сильный ветер",
            AlertType::Humidity { .. } => "💧 Критический уровень влажности",
        };
        
        format!(
            "{}\n\n🏠 Город: {}\n📝 Описание: {}\n\n🌡️ Текущая температура: {}°C\n☁️ Условия: {}\n💨 Ветер: {} км/ч\n💧 Влажность: {}%\n\n⏰ Время: {}",
            alert_type_str,
            weather.location.name,
            alert.description,
            weather.current.temperature,
            weather.current.condition.text,
            weather.current.wind_speed,
            weather.current.humidity,
            chrono::Utc::now().format("%Y-%m-%d %H:%M UTC")
        )
    }
}

pub fn generate_alert_id() -> String {
    Uuid::new_v4().to_string()
}

pub fn create_standard_alert(city: String) -> WeatherAlert {
    WeatherAlert::new(
        generate_alert_id(),
        city.clone(),
        AlertType::StandardWeatherAlert,
        format!("Стандартные предупреждения о погоде для {}", city)
    )
}

pub fn create_temperature_alert(city: String, min: Option<f32>, max: Option<f32>) -> WeatherAlert {
    let description = match (min, max) {
        (Some(min_val), Some(max_val)) => format!("Температура вне диапазона {}°C - {}°C в {}", min_val, max_val, city),
        (Some(min_val), None) => format!("Температура ниже {}°C в {}", min_val, city),
        (None, Some(max_val)) => format!("Температура выше {}°C в {}", max_val, city),
        (None, None) => format!("Контроль температуры в {}", city),
    };
    
    WeatherAlert::new(
        generate_alert_id(),
        city,
        AlertType::TemperatureThreshold { min, max },
        description
    )
}

pub fn create_wind_alert(city: String, max_speed: f32) -> WeatherAlert {
    WeatherAlert::new(
        generate_alert_id(),
        city.clone(),
        AlertType::WindSpeed { max: max_speed },
        format!("Скорость ветра выше {} км/ч в {}", max_speed, city)
    )
}

pub fn create_humidity_alert(city: String, min: Option<u32>, max: Option<u32>) -> WeatherAlert {
    let description = match (min, max) {
        (Some(min_val), Some(max_val)) => format!("Влажность вне диапазона {}% - {}% в {}", min_val, max_val, city),
        (Some(min_val), None) => format!("Влажность ниже {}% в {}", min_val, city),
        (None, Some(max_val)) => format!("Влажность выше {}% в {}", max_val, city),
        (None, None) => format!("Контроль влажности в {}", city),
    };
    
    WeatherAlert::new(
        generate_alert_id(),
        city,
        AlertType::Humidity { min, max },
        description
    )
} 