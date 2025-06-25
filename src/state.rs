use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use teloxide::types::ChatId;

#[derive(Debug, Clone)]
pub struct UserData {
    pub home_town: Option<String>,
    pub interested_towns: Vec<String>,
    pub waiting_for_city: bool,
    pub waiting_for_home_town: bool,
    pub waiting_for_interested_town: bool,
    pub removing_interested_town: bool,
}

impl Default for UserData {
    fn default() -> Self {
        Self {
            home_town: None,
            interested_towns: Vec::new(),
            waiting_for_city: false,
            waiting_for_home_town: false,
            waiting_for_interested_town: false,
            removing_interested_town: false,
        }
    }
}

pub type SharedState = Arc<Mutex<HashMap<ChatId, UserData>>>;

pub fn create_shared_state() -> SharedState {
    Arc::new(Mutex::new(HashMap::new()))
}

pub fn get_user_data(state: &SharedState, chat_id: ChatId) -> UserData {
    let state_guard = state.lock().unwrap();
    state_guard.get(&chat_id).unwrap_or(&UserData::default()).clone()
}

pub fn update_user_data<F>(state: &SharedState, chat_id: ChatId, updater: F) 
where
    F: FnOnce(&mut UserData),
{
    let mut state_guard = state.lock().unwrap();
    let user_data = state_guard.entry(chat_id).or_insert_with(UserData::default);
    updater(user_data);
} 