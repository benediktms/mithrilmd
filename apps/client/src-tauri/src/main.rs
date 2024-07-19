// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod startup;
mod state;
mod system_tray;
mod util;

use tauri::{LogicalSize, Manager, Size, WindowEvent};

use startup::{init_async, init_tracing};
use system_tray::{make_system_tray, system_tray_event_handler};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    dotenvy::dotenv().ok();

    init_tracing();

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
            let _ = tauri::async_runtime::block_on(init_async(app.handle()));

            Ok(())
        })
        .system_tray(make_system_tray())
        .on_system_tray_event(system_tray_event_handler())
        .on_window_event(|global_window_event| {
            let event = global_window_event.event();
            if let WindowEvent::CloseRequested { api, .. } = event {
                global_window_event
                    .window()
                    .hide()
                    .expect("could not hide window");
                api.prevent_close();
            }
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
