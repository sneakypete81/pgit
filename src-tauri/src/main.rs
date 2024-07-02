// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod git;
mod ipc;

use crate::ipc::{command_handler, Ipc};

fn main() {
    tauri::Builder::default()
        .manage(Ipc::default())
        .invoke_handler(command_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
