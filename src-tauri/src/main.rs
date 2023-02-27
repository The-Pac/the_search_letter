#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use crate::command::search_letters;

mod command;
mod model;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![search_letters])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
