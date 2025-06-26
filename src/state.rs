use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use teloxide::types::ChatId;
use crate::storage::Storage;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum AlertType {
    StandardWeatherAlert,
    TemperatureThreshold { min: Option<f32>, max: Option<f32> },
    WindSpeed { max: f32 },
    Humidity { min: Option<u32>, max: Option<u32> },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WeatherAlert {
    pub id: String,
    pub city: String,
    pub alert_type: AlertType,
    pub is_active: bool,
    pub hours_ahead: u8, // За сколько часов предупреждать (6, 12, 24, 48, 72)
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_triggered: Option<chrono::DateTime<chrono::Utc>>,
    pub description: String,
}

impl WeatherAlert {
    pub fn new(id: String, city: String, alert_type: AlertType, description: String, hours_ahead: u8) -> Self {
        Self {
            id,
            city,
            alert_type,
            is_active: true,
            hours_ahead,
            created_at: chrono::Utc::now(),
            last_triggered: None,
            description,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserData {
    pub home_town: Option<String>,
    pub interested_towns: Vec<String>,
    pub weather_alerts: Vec<WeatherAlert>,
    pub waiting_for_city: bool,
    pub waiting_for_forecast_city: bool,
    pub waiting_for_home_town: bool,
    pub waiting_for_interested_town: bool,
    pub removing_interested_town: bool,
    pub waiting_for_alert_city: bool,
    pub waiting_for_alert_temperature_min: bool,
    pub waiting_for_alert_temperature_max: bool,
    pub waiting_for_alert_wind_speed: bool,
    pub waiting_for_alert_humidity_min: bool,
    pub waiting_for_alert_humidity_max: bool,
    pub waiting_for_alert_hours_input: bool,
    pub pending_alert_city: Option<String>,
    pub pending_alert_type: Option<AlertType>,
    pub pending_alert_hours: Option<u8>,
}

impl Default for UserData {
    fn default() -> Self {
        Self {
            home_town: None,
            interested_towns: Vec::new(),
            weather_alerts: Vec::new(),
            waiting_for_city: false,
            waiting_for_forecast_city: false,
            waiting_for_home_town: false,
            waiting_for_interested_town: false,
            removing_interested_town: false,
            waiting_for_alert_city: false,
            waiting_for_alert_temperature_min: false,
            waiting_for_alert_temperature_max: false,
            waiting_for_alert_wind_speed: false,
            waiting_for_alert_humidity_min: false,
            waiting_for_alert_humidity_max: false,
            waiting_for_alert_hours_input: false,
            pending_alert_city: None,
            pending_alert_type: None,
            pending_alert_hours: None,
        }
    }
}

pub struct SharedState {
    pub data: Arc<Mutex<HashMap<ChatId, UserData>>>,
    pub storage: Storage,
}

impl Clone for SharedState {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            storage: self.storage.clone(),
        }
    }
}

pub fn create_shared_state() -> Result<SharedState, Box<dyn std::error::Error>> {
    let storage = Storage::new()?;
    let data = Arc::new(Mutex::new(HashMap::new()));
    Ok(SharedState { data, storage })
}

pub fn create_shared_state_with_data(storage: Storage, loaded_data: Arc<Mutex<HashMap<ChatId, UserData>>>) -> SharedState {
    SharedState { data: loaded_data, storage }
}

pub fn create_test_shared_state() -> Result<SharedState, Box<dyn std::error::Error>> {
    let storage = Storage::new_test()?;
    let data = Arc::new(Mutex::new(HashMap::new()));
    Ok(SharedState { data, storage })
}

pub fn get_user_data(state: &SharedState, chat_id: ChatId) -> UserData {
    let state_guard = state.data.lock().unwrap();
    state_guard.get(&chat_id).unwrap_or(&UserData::default()).clone()
}

pub fn update_user_data<F>(state: &SharedState, chat_id: ChatId, updater: F) 
where
    F: FnOnce(&mut UserData),
{
    let mut state_guard = state.data.lock().unwrap();
    let user_data = state_guard.entry(chat_id).or_insert_with(UserData::default);
    updater(user_data);
    
    // Автоматически сохраняем изменения в Sled
    if let Err(e) = state.storage.save_user_data(chat_id, user_data) {
        log::error!("Failed to save user data for {}: {}", chat_id, e);
    }
} 