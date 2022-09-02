#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use app::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            validate_link,
            open_browser_windows,
            open_browser_linux,
            open_browser_macos
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
