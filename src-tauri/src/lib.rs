
mod config;
mod commands;
mod events;
mod error;
mod types;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(commands::get_handlers())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
