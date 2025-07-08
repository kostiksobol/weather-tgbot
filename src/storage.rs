use std::sync::Arc;
use sled::Db;
use teloxide::types::ChatId;
use teloxide::prelude::Requester;
use crate::state::UserData;

#[derive(Clone)]
pub struct Storage {
    db: Arc<Db>,
}

impl Storage {
    pub fn new() -> Result<Self, sled::Error> {
        // Configure Sled with a specific cache capacity to avoid memory limit errors
        let db = sled::Config::new()
            .path("weather_bot_data")
            .cache_capacity(256 * 1024 * 1024) 
            .open()?;
        log::info!("Sled database opened successfully");
        Ok(Storage { db: Arc::new(db) })
    }
    
    pub fn new_test() -> Result<Self, sled::Error> {
        // Create temporary database for tests
        let test_dir = format!("test_weather_bot_data_{}", std::process::id());
        let db = sled::open(&test_dir)?;
        log::info!("Test Sled database opened: {}", test_dir);
        Ok(Storage { db: Arc::new(db) })
    }
    
    pub fn save_user_data(&self, chat_id: ChatId, user_data: &UserData) -> Result<(), Box<dyn std::error::Error>> {
        let key = chat_id.0.to_le_bytes();
        let value = bincode::serialize(user_data)?;
        self.db.insert(key, value)?;
        self.db.flush()?; // Ensure data is written to disk
        log::debug!("Saved data for user {}", chat_id);
        Ok(())
    }
    
    pub fn load_user_data(&self, chat_id: ChatId) -> Result<Option<UserData>, Box<dyn std::error::Error>> {
        let key = chat_id.0.to_le_bytes();
        if let Some(value) = self.db.get(key)? {
            let user_data: UserData = bincode::deserialize(&value)?;
            log::debug!("Loaded data for user {}", chat_id);
            Ok(Some(user_data))
        } else {
            Ok(None)
        }
    }
    
    pub fn remove_user_data(&self, chat_id: ChatId) -> Result<(), Box<dyn std::error::Error>> {
        let key = chat_id.0.to_le_bytes();
        self.db.remove(key)?;
        self.db.flush()?;
        log::info!("Removed data for user {}", chat_id);
        Ok(())
    }
    
    pub fn load_all_users(&self) -> Result<std::sync::Arc<std::sync::Mutex<std::collections::HashMap<ChatId, UserData>>>, Box<dyn std::error::Error>> {
        let data = std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new()));
        let mut state_guard = data.lock().unwrap();
        
        for result in self.db.iter() {
            let (key, value) = result?;
            let chat_id_bytes: [u8; 8] = key.as_ref().try_into()?;
            let chat_id = ChatId(i64::from_le_bytes(chat_id_bytes));
            let user_data: UserData = bincode::deserialize(&value)?;
            state_guard.insert(chat_id, user_data);
        }
        
        log::info!("Loaded {} users from Sled database", state_guard.len());
        drop(state_guard);
        Ok(data)
    }
    
    pub fn get_all_chat_ids(&self) -> Result<Vec<ChatId>, Box<dyn std::error::Error>> {
        let mut chat_ids = Vec::new();
        
        for result in self.db.iter() {
            let (key, _) = result?;
            let chat_id_bytes: [u8; 8] = key.as_ref().try_into()?;
            let chat_id = ChatId(i64::from_le_bytes(chat_id_bytes));
            chat_ids.push(chat_id);
        }
        
        Ok(chat_ids)
    }
    
    pub async fn cleanup_blocked_users(
        &self,
        bot: &teloxide::Bot,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let chat_ids = self.get_all_chat_ids()?;
        log::info!("Checking {} users for blocked status...", chat_ids.len());
        
        for chat_id in chat_ids {
            match bot.get_chat(chat_id).await {
                Ok(_) => {
                    // User is still accessible, keep their data
                    log::debug!("User {} is still active", chat_id);
                }
                Err(e) => {
                    // Check if error indicates blocked/deleted chat
                    let error_msg = e.to_string().to_lowercase();
                    if error_msg.contains("chat not found") 
                        || error_msg.contains("blocked") 
                        || error_msg.contains("kicked")
                        || error_msg.contains("bot was blocked by the user")
                        || error_msg.contains("forbidden") {
                        
                        // Remove user data
                        self.remove_user_data(chat_id)?;
                        log::info!("Removed data for blocked user: {}", chat_id);
                    } else {
                        log::warn!("Unknown error for user {}: {}", chat_id, e);
                    }
                }
            }
            
            // Small delay to avoid rate limiting
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
        
        Ok(())
    }
    
    pub fn stats(&self) -> Result<(), Box<dyn std::error::Error>> {
        let size = self.db.size_on_disk()?;
        let len = self.db.len();
        log::info!("Database stats: {} users, {} bytes on disk", len, size);
        Ok(())
    }
} 