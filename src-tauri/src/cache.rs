use once_cell::sync::Lazy;
use std::sync::Mutex;

use tokio::sync::mpsc::Sender;

use crate::settings::Settings;

pub static CACHE: Lazy<Mutex<Cache>> = Lazy::new(|| Mutex::new(Cache::default()));

#[derive(Debug)]
pub struct Cache {
    pub settings: Option<Settings>,
    pub secret_key: Option<String>,
    pub sender: Option<Sender<i32>>,
}

impl Default for Cache {
    fn default() -> Self {
        Self {
            settings: None,
            secret_key: None,
            sender: None,
        }
    }
}

pub fn remove_cache() {
    remove_settings_cache();
    remove_secret_key();
}

//==============================================================

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

//==============================================================

pub fn remove_secret_key() {
    let mut cache = CACHE.lock().unwrap();
    cache.secret_key = None;
}

pub fn get_secret_key() -> String {
    let mut cache = CACHE.lock().unwrap();
    if let Some(secret_key) = &cache.secret_key {
        return secret_key.clone();
    }

    let secret_key = format!("wds-{}", rand::random::<u64>().to_string());
    cache.secret_key = Some(secret_key.clone());

    // println!("new secret key: {}", secret_key);

    secret_key
}

//==============================================================

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
