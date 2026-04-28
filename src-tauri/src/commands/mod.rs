
use crate::config::{load_config, save_config};
use crate::error::AppResult;
use crate::types::AppConfig;
use crate::events::CONFIG_CHANGED;
use tauri::{AppHandle, command, Emitter};

#[command]
pub fn get_config(app: AppHandle) -> AppResult<AppConfig> {
    load_config(&app)
}

#[command]
pub fn update_config(app: AppHandle, config: AppConfig) -> AppResult<()> {
    save_config(&app, &config)?;
    app.emit(CONFIG_CHANGED, config).unwrap_or(());
    Ok(())
}

pub fn get_handlers() -> impl Fn(tauri::ipc::Invoke) -> bool {
    tauri::generate_handler![get_config, update_config]
}
