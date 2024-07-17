// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod system_tray;
mod util;

use system_tray::{make_system_tray, system_tray_event_handler};
use tauri::{LogicalSize, Manager, Size};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};
use util::load_from_env;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    let log_fmt = load_from_env::<String>("LOG_FORMAT").unwrap_or("default".into());
    let log_level = load_from_env::<LevelFilter>("LOG_LEVEL").unwrap_or(LevelFilter::TRACE);

    tracing_subscriber::registry()
        .with(if log_fmt == "json" {
            fmt::layer().json().boxed()
        } else if log_fmt == "pretty" {
            fmt::layer()
                .with_thread_ids(true)
                .with_thread_names(true)
                .pretty()
                .boxed()
        } else {
            fmt::layer().boxed()
        })
        .with(
            EnvFilter::builder()
                .with_default_directive(log_level.into())
                .from_env_lossy(),
        )
        .init();

    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            match window.set_size(Size::Logical(LogicalSize {
                width: 1300.0,
                height: 800.0,
            })) {
                Err(err) => println!("{err}"),
                Ok(res) => res,
            }

            Ok(())
        })
        .system_tray(make_system_tray())
        .on_system_tray_event(system_tray_event_handler())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
