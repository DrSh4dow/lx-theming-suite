// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use commands::get_gtk_themes;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_gtk_themes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
