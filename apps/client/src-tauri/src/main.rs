// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod startup;
mod system_tray;
mod util;

use tauri::{LogicalSize, Manager, Size, WindowEvent};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

use startup::startup;
use system_tray::{make_system_tray, system_tray_event_handler};
use util::load_from_env;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    dotenvy::dotenv().ok();

    let log_fmt = load_from_env::<String>("RUST_LOG").unwrap_or("default".into());
    let log_level = load_from_env::<LevelFilter>("LOG_LEVEL").unwrap_or(LevelFilter::TRACE);

    tracing_subscriber::registry()
        .with(if log_fmt == "json" {
            fmt::layer()
                .with_thread_ids(true)
                .with_thread_names(true)
                .json()
                .boxed()
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
            let _ = tauri::async_runtime::block_on(startup(app.handle()));

            Ok(())
        })
        .system_tray(make_system_tray())
        .on_system_tray_event(system_tray_event_handler())
        .on_window_event(|e| {
            let event = e.event();
            if let WindowEvent::CloseRequested { api, .. } = event {
                e.window().hide().expect("could not hide window");
                api.prevent_close();
            }
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
