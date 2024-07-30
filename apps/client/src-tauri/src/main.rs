// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::{
    startup::{init_async, init_tracing},
    system_tray::{make_system_tray, system_tray_event_handler},
    vault::*,
};
use tauri::{LogicalSize, Manager, Size, WindowEvent};

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
        .invoke_handler(tauri::generate_handler![
            setup_new_vault,
            get_all_vaults,
            find_vault
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use app::vault::*;
    use specta::collect_types;
    use tauri_specta::ts;

    #[test]
    fn export_bindings() {
        ts::export(
            collect_types![setup_new_vault, get_all_vaults, find_vault],
            "../src/types/bindings.ts",
        )
        .unwrap();
    }
}
