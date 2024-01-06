// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use commands::get_gtk_themes;

mod commands;
mod state;

fn main() {
    // Initializes logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Initializes tauri
    tauri::Builder::default()
        .manage(state::GTKThemeStorage {
            themes: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![get_gtk_themes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
