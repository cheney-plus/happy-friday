
use crate::error::{AppError, AppResult};
use crate::types::AppConfig;
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

pub fn get_config_path(app: &AppHandle) -> AppResult<PathBuf> {
    let app_dir = app.path().app_data_dir()
        .map_err(|_| AppError::Config("Failed to get app data dir".into()))?;
    
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir)?;
    }
    
    Ok(app_dir.join("config.json"))
}

pub fn load_config(app: &AppHandle) -> AppResult<AppConfig> {
    let config_path = get_config_path(app)?;
    
    if !config_path.exists() {
        let default_config = AppConfig::default();
        save_config(app, &default_config)?;
        return Ok(default_config);
    }
    
    let content = fs::read_to_string(&config_path)?;
    let config = serde_json::from_str(&content)?;
    Ok(config)
}

pub fn save_config(app: &AppHandle, config: &AppConfig) -> AppResult<()> {
    let config_path = get_config_path(app)?;
    let content = serde_json::to_string_pretty(config)?;
    fs::write(&config_path, content)?;
    Ok(())
}
