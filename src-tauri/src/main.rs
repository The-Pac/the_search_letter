#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use simple_logger::SimpleLogger;

use crate::command::{get_all_extensions, search_letters};

mod command;
mod model;

fn main() {
    SimpleLogger::new().with_colors(true).init().expect("Cannot start the logger");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![search_letters,get_all_extensions])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
