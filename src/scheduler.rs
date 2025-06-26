use std::time::Duration;
use tokio::time::interval;
use teloxide::{Bot, prelude::Requester};
use crate::state::{SharedState, update_user_data};
use crate::alerts::AlertChecker;
use crate::weather_api::get_current_weather;

pub struct AlertScheduler {
    bot: Bot,
    state: SharedState,
}

impl AlertScheduler {
    pub fn new(bot: Bot, state: SharedState) -> Self {
        Self { bot, state }
    }
    
    pub async fn start(&self) {
        let mut interval = interval(Duration::from_secs(300)); // Проверяем каждые 5 минут
        
        loop {
            interval.tick().await;
            if let Err(e) = self.check_all_alerts().await {
                log::error!("Error checking alerts: {}", e);
            }
        }
    }
    
    async fn check_all_alerts(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        log::info!("Starting alert check cycle...");
        
        // Получаем копию всех данных пользователей
        let users_data = {
            let state_guard = self.state.data.lock().unwrap();
            state_guard.clone()
        };
        
        for (chat_id, user_data) in users_data {
            for alert in &user_data.weather_alerts {
                if !alert.is_active {
                    continue;
                }
                
                // Проверяем, не было ли недавно срабатывания (избегаем спама)
                if let Some(last_triggered) = alert.last_triggered {
                    let time_since_last = chrono::Utc::now() - last_triggered;
                    if time_since_last.num_minutes() < 60 { // Минимум час между срабатываниями
                        continue;
                    }
                }
                
                match AlertChecker::check_alert(alert).await {
                    Ok(true) => {
                        log::info!("Alert triggered for user {} in city {}", chat_id, alert.city);
                        
                        // Получаем погодные данные для уведомления
                        match get_current_weather(&alert.city).await {
                            Ok(weather) => {
                                let message = AlertChecker::format_alert_message(alert, &weather);
                                
                                if let Err(e) = self.bot.send_message(chat_id, message).await {
                                    log::error!("Failed to send alert to user {}: {}", chat_id, e);
                                } else {
                                    // Обновляем время последнего срабатывания
                                    update_user_data(&self.state, chat_id, |user_data| {
                                        if let Some(alert_to_update) = user_data.weather_alerts.iter_mut()
                                            .find(|a| a.id == alert.id) {
                                            alert_to_update.last_triggered = Some(chrono::Utc::now());
                                        }
                                    });
                                }
                            }
                            Err(e) => {
                                log::error!("Failed to fetch weather data for alert in {}: {}", alert.city, e);
                            }
                        }
                    }
                    Ok(false) => {
                        // Алерт не сработал, это нормально
                    }
                    Err(e) => {
                        log::error!("Error checking alert for {} in {}: {}", chat_id, alert.city, e);
                    }
                }
                
                // Небольшая задержка между проверками чтобы не перегружать API
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }
        
        log::info!("Alert check cycle completed");
        Ok(())
    }
} 