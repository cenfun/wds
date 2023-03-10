use serde::{Deserialize, Serialize};

use tauri::{PhysicalPosition, Position::Physical};

use crate::common::get_data_file_object;
use crate::common::save_data_file_object;
use crate::common::WINDOW;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowState {
    pub x: i32,
    pub y: i32,
}

impl Default for WindowState {
    fn default() -> Self {
        Self { x: 100, y: 100 }
    }
}

pub fn save_window_state(window_state: WindowState) -> bool {
    save_data_file_object("window_state.json", window_state)
}

pub fn get_window_state() -> WindowState {
    let res = get_data_file_object("window_state.json");
    match res {
        Ok(s) => s,
        Err(_) => WindowState::default(),
    }
}

//===============================================================================================

pub fn init_window_state() {
    let window_state = get_window_state();

    let mut x = window_state.x;
    let mut y = window_state.y;

    let win = WINDOW.get().expect("window is not initialized");

    //https://github.com/tauri-apps/tauri-plugin-positioner
    let screen = win.current_monitor().unwrap().unwrap();

    let screen_position = screen.position();
    let screen_x = screen_position.x as i32;
    let screen_y = screen_position.y as i32;

    let screen_size = screen.size();
    let screen_width = screen_size.width as i32;
    let screen_height = screen_size.width as i32;

    let window_size = win.outer_size().unwrap();
    let window_width = window_size.width as i32;
    let window_height = window_size.width as i32;

    if x + window_width > screen_x + screen_width {
        x = screen_x + screen_width - window_width;
    } else if x < 0 {
        x = 0;
    }

    if y + window_height > screen_y + screen_height {
        y = screen_y + screen_height - window_height;
    } else if y < 0 {
        y = 0;
    }

    let physical_pos = PhysicalPosition { x, y };

    win.set_position(Physical(physical_pos)).unwrap();

    win.show().unwrap();

    win.set_focus().unwrap();
}
