// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod system_tray;

use system_tray::{make_system_tray, system_tray_event_handler};
use tauri::{LogicalSize, Manager, Size};

fn main() {
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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
