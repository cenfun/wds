#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{env, time::Duration};

use tauri::{generate_context, generate_handler, Builder, Manager, WindowEvent};

mod cache;

mod common;
use common::APP;
use common::WINDOW;

mod command;
use command::invoke_get_settings;
use command::invoke_restart;
use command::invoke_save_dir;
use command::invoke_save_port;
use command::invoke_save_profile;

mod server;
use server::start_server;

mod settings;

mod utils;
//use utils::log_green;

mod window_state;
use window_state::init_window_state;
use window_state::save_window_state;
use window_state::WindowState;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    env::set_var("RUST_BACKTRACE", "1");

    start_app();
}

fn start_app() {
    let on_window_moved = fns::debounce(
        |pos| {
            let (x, y) = pos;
            //println!("x {} y {}", x, y);

            let window_state = WindowState { x, y };

            save_window_state(window_state);
        },
        Duration::from_millis(500),
    );

    Builder::default()
        .setup(|app| {
            //init app
            APP.set(app.handle()).unwrap();

            //init window
            let win = app.get_window("main").unwrap();
            win.set_focus().unwrap();
            WINDOW.set(win).unwrap();

            init_window_state();

            start_server();

            Ok(())
        })
        .invoke_handler(generate_handler![
            invoke_get_settings,
            invoke_restart,
            invoke_save_dir,
            invoke_save_port,
            invoke_save_profile
        ])
        .on_page_load(|_window, _payload| {
            //log_green("page loaded");
        })
        .on_window_event(move |event| match event.event() {
            WindowEvent::Moved(pos) => {
                //println!("{:?}", pos);
                on_window_moved.call((pos.x, pos.y));
            }
            WindowEvent::Destroyed => {
                println!("destroyed")
            }
            _ => {}
        })
        .run(generate_context!())
        .expect("error while running application");
}
