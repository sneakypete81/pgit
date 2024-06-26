// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;

use tauri::Window;

#[derive(Clone, serde::Serialize)]
struct Payload {
    branches: Vec<String>,
}

#[tauri::command]
fn init(window: Window) {
    std::thread::spawn(move || {
        let mut branches = vec!["main".into(), "private/pete/feature".into()];
        loop {
            branches.push(format!("another-branch-{}", branches.len()));
            send_branches(&window, branches.clone());
            std::thread::sleep(Duration::from_secs(1));
        }
    });
}

fn send_branches(window: &Window, branches: Vec<String>) {
    window.emit("branches", Payload { branches }).unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![init])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
