use crate::state::{WeatherAlert, AlertType};
use crate::weather_api::{CurrentWeather, get_current_weather, get_forecast, ForecastResponse};
use uuid::Uuid;

pub struct AlertChecker;

impl AlertChecker {
    pub async fn check_alert(alert: &WeatherAlert) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        // Используем прогноз для заблаговременного предупреждения
        Self::check_forecast_alert(alert).await
    }
    
    pub async fn check_forecast_alert(alert: &WeatherAlert) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        // Получаем прогноз на 3 дня (максимум что поддерживает API)
        let forecast = get_forecast(&alert.city, 3).await?;
        
        // Проверяем прогноз на время, указанное в alert.hours_ahead
        Self::check_forecast_for_hours(&forecast, alert)
    }
    
    fn check_forecast_for_hours(forecast: &ForecastResponse, alert: &WeatherAlert) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let target_hours = alert.hours_ahead as usize;
        
        // Для простоты проверяем дневные показатели
        // В реальности можно было бы проверять почасовые данные
        let days_ahead = if target_hours <= 24 { 0 } else if target_hours <= 48 { 1 } else { 2 };
        
        if let Some(forecast_day) = forecast.forecast.forecast_day.get(days_ahead) {
            match &alert.alert_type {
                AlertType::StandardWeatherAlert => {
                    Self::check_forecast_standard_conditions(forecast_day)
                }
                AlertType::TemperatureThreshold { min, max } => {
                    let min_temp = forecast_day.day.min_temp;
                    let max_temp = forecast_day.day.max_temp;
                    
                    let triggered = if let Some(min_threshold) = min {
                        min_temp < *min_threshold
                    } else {
                        false
                    } || if let Some(max_threshold) = max {
                        max_temp > *max_threshold
                    } else {
                        false
                    };
                    Ok(triggered)
                }
                AlertType::WindSpeed { max } => {
                    Ok(forecast_day.day.max_wind > *max)
                }
                AlertType::Humidity { min, max } => {
                    let avg_humidity = forecast_day.day.avg_humidity as u32;
                    let triggered = if let Some(min_hum) = min {
                        avg_humidity < *min_hum
                    } else {
                        false
                    } || if let Some(max_hum) = max {
                        avg_humidity > *max_hum
                    } else {
                        false
                    };
                    Ok(triggered)
                }
            }
        } else {
            Ok(false) // Недостаточно данных прогноза
        }
    }
    
    // Старая функция для совместимости и проверки текущего статуса
    pub async fn check_current_alert(alert: &WeatherAlert) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
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
    
    fn check_forecast_standard_conditions(forecast_day: &crate::weather_api::ForecastDay) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let condition_text = forecast_day.day.condition.text.to_lowercase();
        let min_temp = forecast_day.day.min_temp;
        let max_temp = forecast_day.day.max_temp;
        let max_wind = forecast_day.day.max_wind;
        
        // Список ключевых слов для экстремальных погодных условий
        let extreme_conditions = [
            "thunderstorm", "storm", "tornado", "hurricane", "cyclone",
            "blizzard", "hail", "snow", "ice", "freeze", "freezing",
            "extreme", "severe", "heavy", "violent", "dangerous"
        ];
        
        // Проверяем условия
        let extreme_weather = extreme_conditions.iter()
            .any(|&condition| condition_text.contains(condition));
        
        let extreme_temperature = max_temp > 40.0 || min_temp < -20.0;
        let extreme_wind = max_wind > 50.0;
        
        Ok(extreme_weather || extreme_temperature || extreme_wind)
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
}

pub fn generate_alert_id() -> String {
    Uuid::new_v4().to_string()
}

pub fn create_standard_alert(city: String, hours_ahead: u8) -> WeatherAlert {
    WeatherAlert::new(
        generate_alert_id(),
        city,
        AlertType::StandardWeatherAlert,
        hours_ahead
    )
}

pub fn create_temperature_alert(city: String, min: Option<f32>, max: Option<f32>, hours_ahead: u8) -> WeatherAlert {
    WeatherAlert::new(
        generate_alert_id(),
        city,
        AlertType::TemperatureThreshold { min, max },
        hours_ahead
    )
}

pub fn create_wind_alert(city: String, max_speed: f32, hours_ahead: u8) -> WeatherAlert {
    WeatherAlert::new(
        generate_alert_id(),
        city,
        AlertType::WindSpeed { max: max_speed },
        hours_ahead
    )
}

pub fn create_humidity_alert(city: String, min: Option<u32>, max: Option<u32>, hours_ahead: u8) -> WeatherAlert {
    WeatherAlert::new(
        generate_alert_id(),
        city,
        AlertType::Humidity { min, max },
        hours_ahead
    )
} 