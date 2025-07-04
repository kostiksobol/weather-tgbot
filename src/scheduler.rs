use std::time::Duration;
use tokio::time::interval;
use teloxide::{Bot, prelude::Requester};
use crate::state::SharedState;
use crate::weather_api::get_current_weather;
use crate::alerts::AlertChecker;

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
                
                if AlertChecker::check_forecast_alert(alert).await.unwrap_or(false) {
                    if let Ok(_weather) = get_current_weather(&alert.city).await {
                        // Локализация здесь затруднена, т.к. нет прямого доступа к языку пользователя.
                        // Отправляем простое уведомление. Пользователь может проверить детали в боте.
                        let message = format!(
                            "⚠️ WEATHER ALERT in {}! ⚠️\nCheck the bot for more details.",
                            alert.city
                        );
                        
                        if let Err(e) = self.bot.send_message(chat_id, message).await {
                            log::error!("Failed to send alert to {}: {}", chat_id, e);
                        }
                        
                        // Обновляем время последнего срабатывания
                        self.state.data.lock().unwrap()
                            .get_mut(&chat_id)
                            .and_then(|user_data| user_data.weather_alerts.iter_mut().find(|a| a.id == alert.id))
                            .map(|a| a.last_triggered = Some(chrono::Utc::now()));
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