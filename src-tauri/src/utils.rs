use std::{thread::sleep, time::Duration};

use serde::Serialize;
use tauri::Emitter;

use crate::common::WINDOW;

// ================================================================================================

#[allow(dead_code)]
pub fn delay(ms: u64) {
    sleep(Duration::from_millis(ms));
}

// ================================================================================================

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, Serialize)]
pub struct Message {
    pub color: String,
    pub value: String,
}

pub fn log_color<T: Into<String>>(input: T, color: &str) {
    let s = input.into();
    let c = color.into();
    match c {
        //'black'
        "red" => println!("\x1b[91m{}\x1b[0m", s),
        "green" => println!("\x1b[92m{}\x1b[0m", s),
        "yellow" => println!("\x1b[93m{}\x1b[0m", s),
        "blue" => println!("\x1b[94m{}\x1b[0m", s),
        "magenta" => println!("\x1b[95m{}\x1b[0m", s),
        "cyan" => println!("\x1b[96m{}\x1b[0m", s),
        //'white'
        _ => println!("{}", s),
    }

    if let Some(win) = WINDOW.get() {
        let _ = win.emit(
            "message",
            Message {
                color: c.to_string(),
                value: s,
            },
        );
    }
}

#[allow(dead_code)]
pub fn log_red<T: Into<String>>(input: T) {
    log_color(input, "red")
}

#[allow(dead_code)]
pub fn log_green<T: Into<String>>(input: T) {
    log_color(input, "green")
}

#[allow(dead_code)]
pub fn log_yellow<T: Into<String>>(input: T) {
    log_color(input, "yellow")
}

#[allow(dead_code)]
pub fn log_blue<T: Into<String>>(input: T) {
    log_color(input, "blue")
}

#[allow(dead_code)]
pub fn log_magenta<T: Into<String>>(input: T) {
    log_color(input, "magenta")
}

#[allow(dead_code)]
pub fn log_cyan<T: Into<String>>(input: T) {
    log_color(input, "cyan")
}

#[allow(dead_code)]
pub fn log<T: Into<String>>(input: T) {
    log_color(input, "")
}
