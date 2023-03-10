// use crate::common::Error;

use crate::settings::get_settings;
use crate::settings::save_dir;
use crate::settings::save_port;
use crate::settings::save_profile;
use crate::settings::DirItem;
use crate::settings::ProfileItem;
use crate::settings::Settings;

use crate::server::restart;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

//========================================================
//settings

#[tauri::command(async)]
pub fn invoke_get_settings() -> Settings {
    get_settings()
}

#[tauri::command(async)]
pub fn invoke_save_port(port: u16) -> bool {
    save_port(port)
}

#[tauri::command(async)]
pub fn invoke_save_profile(action: &str, data: ProfileItem) -> bool {
    save_profile(action, data)
}

#[tauri::command(async)]
pub fn invoke_save_dir(action: &str, id: String, data: DirItem) -> bool {
    save_dir(action, id, data)
}

#[tauri::command(async)]
pub async fn invoke_restart() -> bool {
    restart().await
}
