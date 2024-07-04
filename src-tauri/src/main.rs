// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod git;
mod ipc;

use crate::ipc::Ipc;

fn main() {
    let (invoke_handler, register_events) = ipc::build();
    tauri::Builder::default()
        .manage(Ipc::default())
        .invoke_handler(invoke_handler)
        .setup(|app| {
            register_events(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
