use once_cell::sync::Lazy;
use std::sync::Mutex;

use tokio::sync::mpsc::Sender;

use crate::settings::Settings;

pub static CACHE: Lazy<Mutex<Cache>> = Lazy::new(|| Mutex::new(Cache::default()));

#[derive(Debug)]
pub struct Cache {
    pub settings: Option<Settings>,
    pub sender: Option<Sender<i32>>,
}

impl Default for Cache {
    fn default() -> Self {
        Self {
            settings: None,
            sender: None,
        }
    }
}

pub fn remove_settings_cache() {
    let mut cache = CACHE.lock().unwrap();
    cache.settings = None;
}

pub fn get_settings_cache() -> Option<Settings> {
    let cache = CACHE.lock().unwrap();
    if let Some(settings) = &cache.settings {
        return Some(settings.clone());
    }
    None
}

pub fn update_settings_cache(settings: Settings) {
    let mut cache = CACHE.lock().unwrap();
    cache.settings = Some(settings);
}

pub fn update_sender(sender: Sender<i32>) {
    let mut cache = CACHE.lock().unwrap();
    cache.sender = Some(sender);
}

pub fn get_sender() -> Option<Sender<i32>> {
    let cache = CACHE.lock().unwrap();
    if let Some(sender) = &cache.sender {
        return Some(sender.clone());
    }
    None
}
