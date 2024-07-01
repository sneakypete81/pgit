// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod branches;
mod git;

use crate::branches::BranchEmitter;
use tauri::{Result, Window};

#[tauri::command]
fn init(branch_emitter: tauri::State<BranchEmitter>, window: Window) -> Result<()> {
    window.show()?;
    branch_emitter.start(window);
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(BranchEmitter::default())
        .invoke_handler(tauri::generate_handler![init])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
