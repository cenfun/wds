#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{env, time::Duration};

use tauri::{generate_context, generate_handler, Builder, Manager, WindowEvent};

mod cache;

mod common;

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
use utils::log_red;

mod app;
use app::on_init_before;
use app::on_page_load;
use app::on_window_moved;

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    env::set_var("RUST_BACKTRACE", "1");
    start_app();
}

fn start_app() {
    let _on_window_moved = fns::debounce(|pos| on_window_moved(pos), Duration::from_millis(500));
    let _on_page_load = fns::debounce(|_| on_page_load(), Duration::from_millis(500));
    Builder::default()
        .setup(|app| {
            on_init_before(app);
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
        .on_page_load(move |_window, _payload| _on_page_load.call(true))
        .on_window_event(move |event| match event.event() {
            WindowEvent::Moved(pos) => _on_window_moved.call((pos.x, pos.y)),
            WindowEvent::Destroyed => println!("destroyed"),
            _ => {}
        })
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            log_red(format!("existed instance: {:?}", argv));
            app.emit_all("single-instance", Payload { args: argv, cwd })
                .unwrap();
        }))
        .run(generate_context!())
        .expect("error while running application");
}
