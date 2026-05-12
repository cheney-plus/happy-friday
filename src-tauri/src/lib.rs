use tauri::Manager;
mod config;
mod commands;
mod db;
mod events;
mod error;
mod llm;
mod types;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            db::init_db(app.handle())?;

            #[cfg(target_os = "linux")]
            {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.set_decorations(false);
                }
            }
            Ok(())
        })
        .invoke_handler(commands::get_handlers())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
