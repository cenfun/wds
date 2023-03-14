use std::fs::create_dir;
use std::fs::read_to_string;
use std::fs::write;
use std::path::PathBuf;

use once_cell::sync::OnceCell;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;

use tauri::Window;
pub static WINDOW: OnceCell<Window> = OnceCell::new();

use tauri::AppHandle;
pub static APP: OnceCell<AppHandle> = OnceCell::new();

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(m: impl Into<String>) -> Self {
        Self { message: m.into() }
    }
}

//=============================================================================================

pub fn get_default_file_path(filename: impl Into<String>) -> PathBuf {
    let app = APP.get().expect("app is not initialized");
    let resource_file = format!("../assets/{}", filename.into());
    app.path_resolver().resolve_resource(resource_file).unwrap()
}

pub fn get_data_dir() -> PathBuf {
    let app = APP.get().expect("app is not initialized");
    let data_dir = app.path_resolver().app_data_dir().unwrap();

    //create data dir if not exist
    if !data_dir.exists() {
        create_dir(&data_dir).unwrap();
    }

    data_dir
}

//=============================================================================================
//data file api

pub fn get_data_file_path(filename: impl Into<String>) -> PathBuf {
    let data_dir = get_data_dir();
    data_dir.join(filename.into())
}

fn get_data_file_string(filename: impl Into<String>) -> Result<String, Error> {
    let filename = filename.into();
    let mut file_path = get_data_file_path(&filename);
    if !file_path.exists() {
        let default_path = get_default_file_path(&filename);
        if !default_path.exists() {
            return Err(Error::new(format!("Not found file: {:?}", file_path)));
        }
        file_path = default_path;
    }

    match read_to_string(file_path) {
        Ok(s) => Ok(s),
        Err(e) => Err(Error::new(e.to_string())),
    }
}

pub fn get_data_file_object<T>(filename: impl Into<String>) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let s = get_data_file_string(filename.into())?;
    match serde_json::from_str(&s) {
        Ok(o) => Ok(o),
        Err(e) => Err(Error::new(e.to_string())),
    }
}

fn save_data_file_string(filename: String, contents: String) -> bool {
    let data_file_path = get_data_file_path(filename);
    match write(data_file_path, contents) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn save_data_file_object<T>(filename: impl Into<String>, object: T) -> bool
where
    T: Serialize,
{
    match to_string_pretty(&object) {
        Ok(s) => save_data_file_string(filename.into(), s),
        Err(_) => false,
    }
}
